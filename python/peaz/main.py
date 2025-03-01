import os
import requests
from bs4 import BeautifulSoup

GITHUB_RELEASES_URL = "https://github.com/peass-ng/PEASS-ng/releases/latest"
OUTPUT_FILE = "linpeas.sh"

def get_latest_linpeas_url():
    """Scrape the latest LinPEAS release URL from GitHub."""
    print("Fetching latest LinPEAS release URL...")

    try:
        response = requests.get(GITHUB_RELEASES_URL, timeout=10)
        response.raise_for_status()

        # Extract the redirected URL (which contains the latest version tag)
        latest_release_url = response.url
        version_tag = latest_release_url.split("/")[-1]
        
        linpeas_url = f"https://github.com/peass-ng/PEASS-ng/releases/download/{version_tag}/linpeas.sh"
        print(f"Latest LinPEAS version: {version_tag}")
        return linpeas_url
    except requests.RequestException as e:
        print(f"Error fetching latest release: {e}")
        exit(1)

def download_linpeas(url):
    """Download LinPEAS and make it executable."""
    print(f"Downloading LinPEAS from {url}...")

    try:
        response = requests.get(url, timeout=10)
        response.raise_for_status()

        with open(OUTPUT_FILE, "w", encoding="utf-8") as file:
            file.write(response.text)

        os.chmod(OUTPUT_FILE, 0o755)
        print(f"LinPEAS saved as {OUTPUT_FILE} and made executable.")
    except requests.RequestException as e:
        print(f"Error downloading LinPEAS: {e}")
        exit(1)

if __name__ == "__main__":
    latest_linpeas_url = get_latest_linpeas_url()
    download_linpeas(latest_linpeas_url)
