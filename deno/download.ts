import { dirname, join, normalize } from "https://deno.land/std@0.218.2/path/mod.ts";
import { ensureDir } from "https://deno.land/std@0.218.2/fs/mod.ts";
import { UntarStream } from "jsr:@std/tar/untar-stream";
import * as zip from "jsr:@quentinadam/zip";

const HOME_DIR = Deno.env.get("HOME");
if (!HOME_DIR) {
  console.error("Could not determine HOME directory.");
  Deno.exit(1);
}

const BIN_DIR = join(HOME_DIR, ".bin");
await ensureDir(BIN_DIR);

const FILES = [
  { url: "https://github.com/epi052/feroxbuster/releases/download/v2.11.0/x86_64-linux-feroxbuster.tar.gz", name: "x86_64-linux-feroxbuster.tar.gz", executable: "feroxbuster" },
  { url: "https://caido.download/releases/v0.46.0/caido-cli-v0.46.0-linux-x86_64.tar.gz", name: "caido-cli-v0.46.0-linux-x86_64.tar.gz", executable: "caido-cli" },
  { url: "https://github.com/roapi/roapi/releases/download/roapi-v0.12.4/roapi-x86_64-unknown-linux-musl.tar.gz", name: "roapi-x86_64-unknown-linux-musl.tar.gz", executable: "roapi" },
  { url: "https://github.com/projectdiscovery/subfinder/releases/download/v2.7.0/subfinder_2.7.0_linux_amd64.zip", name: "subfinder_2.7.0_linux_amd64.zip", executable: "subfinder" },
];

async function downloadFile(url: string, outputPath: string) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to download ${url}: ${response.statusText}`);
  }

  const contentLength = response.headers.get("content-length");
  const totalSize = contentLength ? parseInt(contentLength, 10) : 0;
  let receivedSize = 0;

  const file = await Deno.create(outputPath);
  const reader = response.body?.getReader();
  if (!reader) throw new Error("Failed to get reader from response body");

  const decoder = new TextDecoder();
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    await file.write(value);
    receivedSize += value.length;
    if (totalSize) {
      const percent = ((receivedSize / totalSize) * 100).toFixed(2);
      console.log(`Downloading: ${percent}%`);
    }
  }
  file.close();
}

async function postDownload(filePath: string, executableName: string) {
  const extractPath = dirname(filePath);
  
  if (filePath.endsWith(".tar.gz")) {
    const file = await Deno.open(filePath, { read: true });
    const stream = file.readable.pipeThrough(new DecompressionStream("gzip")).pipeThrough(new UntarStream());
    
    for await (const entry of stream) {
      const path = join(extractPath, normalize(entry.path));
      await Deno.mkdir(dirname(path), { recursive: true });
      const outputFile = await Deno.create(path);
      if (entry.readable) {
        await entry.readable.pipeTo(outputFile.writable);
      }
    }
    //file.close();
    await Deno.remove(filePath);
  } else if (filePath.endsWith(".zip")) {
    await zip.extract(filePath, extractPath);
    await Deno.remove(filePath);
  }
  
  const executablePath = join(BIN_DIR, executableName);
  await Deno.chmod(executablePath, 0o755);
  console.log(`Set executable permissions: ${executablePath}`);
}

for (const file of FILES) {
  const outputPath = join(BIN_DIR, file.name);
  console.log(`Downloading ${file.url} to ${outputPath}`);
  await downloadFile(file.url, outputPath);
  await postDownload(outputPath, file.executable);
  console.log(`Processed ${file.name}`);
}

console.log("All files downloaded and processed successfully.");
