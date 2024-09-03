import requests
import bs4

URL="https://www.ffvbbeach.org/ffvbapp/resu/vbspo_calendrier.php?saison=2024/2025&codent=LIIDF&poule=PMA"
CERTIFICATE="/etc/ssl/certs/www-ffvbbeach-org-chain.pem"
BS4_PARSER="html.parser"

def main() -> int:
    # Request to the match page
    r = requests.get(URL, verify=CERTIFICATE)

    # Parse with soup the content of the page
    soup = bs4.BeautifulSoup(r.content, BS4_PARSER)

    # Fine all the match lines
    lines = soup.find_all("tr")

    # Filter the lines to get only lines with matches
    match_lines = filter(lambda l: len(l.contents) == 10, lines)

    # Print the match informations
    for match in match_lines:
        print(f"{match.contents[3].text} vs {match.contents[5].text}")

    return 0

if __name__ == "__main__":
    main()