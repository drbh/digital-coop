import requests
import lxml.html as lh
import json

url = "https://www.foodcoop.com/produce"

page = requests.get(url,verify=False)
doc = lh.fromstring(page.content)

table = doc.xpath('//*[@id="main"]/div/div/div[4]/table')[0]

produce = []
for elem in table:
    cells = elem.getchildren()
    for cell in cells:
        v = []
        for c in cell.getchildren():
            for x in c.getchildren():
                name = x.text.strip()
                v.append(name)
                break
            v.append(c.text.strip())
            
        if len(v) > 2:
            m = [v[0], *v[2:]]
            produce.append(m)


print(json.dumps(produce,indent=4))



import requests
import re
  
# api-endpoint 
URL = "http://127.0.0.1:8080/graphql"

def execute_gql(gql):
    gql = " ".join(gql.split())
    gql = re.sub(r'"', '\\"', gql)
    query = '{{"query": "{}"}}'.format(gql)
    # query
    print(query)

    headers = {'content-type': 'application/json'}

    r = requests.post(
        url = URL,
        data = query,
        headers = headers
    )
    print(r.content)
    data = r.json().get("data")
    return data

for p in produce:
    name = p[0]
    sku = "idk"
    unit = p[1]
    price = p[2]
    organic = p[3]
    brand = p[4]
    prod_mut = (
        'mutation {'
        '  createItem(newItem: {name: "'+name+'", unit: "'+unit+'",organic: "'+organic+'", brand: "'+brand+'" sku: "'+sku+'", price: '+str(float(price[1:]))+'}) {'
        '    name'
        '    sku'
        '    price'
        '  }'
        '}'
    )
    data = execute_gql(prod_mut)
    print(data)