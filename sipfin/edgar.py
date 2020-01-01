import requests as r
import bs4
import pandas as pd


"""
list companies
get company ids
"""
search = "https://www.sec.gov/cgi-bin/browse-edgar?company=Two+Sigma&owner=exclude&action=getcompany&count=100"

ROOT = 'https://www.sec.gov'

def search_link(name: str, form_type='13F-HR') -> str:
    company = name.replace(' ', '+')
    return ROOT + '/cgi-bin/browse-edgar?action=getcompany' + '&company=' + company + '&type=' + form_type + '&count=100'  # + '&output=xml'


def get_page(link: str, parser:str='html.parser') -> bs4.BeautifulSoup:
    return bs4.BeautifulSoup(r.get(link).text, 'html.parser')


def next_pages(page: bs4.BeautifulSoup) -> list:
    pages = [page]
    next_page = page.find('span', {'id': 'next'})
    while next_page is not None:
        page = get_page(next_page.a['href'])
        pages.append(page)
        next_page = page.find('span', {'id': 'next'})

    return pages


def grab_docs_links(page: bs4.BeautifulSoup, output:str='dict'):
    """

    """
    print(f'getting document links')
    if output == 'dict':
        docs = {}
    elif output == 'list':
        docs = []

    all_links = page.find_all('a', {'id': 'documentsbutton'})

    for l in all_links:
        cur_page = get_page(ROOT + l['href'])
        date = cur_page.find('div', {'class' : 'info'}).text
        cur_table = cur_page.find('table', {'class': 'tableFile'})
        links = cur_table.find_all('a')

        if len(links) < 4:
            continue
        
        html_link = ROOT + links[2]['href']

        if output == 'dict':
            docs[date] = html_link
        elif output == 'list':
            docs.append(html_link)
    return docs


def get_holding_from_link(link: str)-> pd.DataFrame:
    # link is html formatted 13F-HR form link
    p = get_page(link)
    df = get_holding(p)
    return df


def get_holding(page: bs4.BeautifulSoup, output='df') -> pd.DataFrame:
    table = page.find('table', {'summary': 'Form 13F-NT Header Information'})

    if output == 'df':
        ret = clean_holding(pd.read_html(table.prettify())[0])

    return ret


def clean_holding(df: pd.DataFrame) -> pd.DataFrame:
    df.columns = df.iloc[2]
    df.drop([0, 1, 2], inplace=True)
    return df


def company_history(name:str, verbose:bool=False)-> dict:
    """
    date : df
    """
    history = {}
    link = search_link(name)
    page = get_page(link)
    doc_links = grab_docs_links(page, output='dict')
    for date, doc_link in doc_links.items():
        print(f'date: {date}')
        doc_page = get_page(doc_link)
        df = get_holding(doc_page)
        history[date] = df
    return history


def main():
    """
    1. given list of companies, gather all of the CIKs using cik_lookup (might need to use selenium :(  ))
    2. for each CIK, gather links to all 13F-HR forms html formatting, txt link if html nonexistent
    3. for each html page, create dataframe 
    4. dictionary of companies:
        "search_term" : [
            cik : [
                date : dataframe
            ]
        ]
    """

    companies = ['Renaissance Technologies']
    # companies = ['Renaissance Technologies', 'Two Sigma Investments', 'Bridgewater Associates',
    #              'AQR Capital Management', 'Millennium Management', 'Elliott Management', 'BlackRock', 'Citadel LLC']
    data = {name: company_history(name) for name in companies}
    return data


if __name__ == "__main__":

    data = main()
    print(data)
