from urllib import request
import sys,json

def create_book(hostname):
    req = request.Request(hostname + '/books', method='POST')
    req.add_header('Content-Type', 'application/json')
    body = json.dumps({
        'title': 'Twenty Thousand Leagues Under the Seas: A World Tour Underwater',
        'author': 'Jules Verne'
    })
    return request.urlopen(req, data=body.encode())

def get_books(hostname):
    req = request.Request(hostname + '/books', method='GET')
    req.add_header('Accept', 'application/json')
    return request.urlopen(req)

def delete_book(hostname, book_id):
    req = request.Request(hostname + '/books/' + str(book_id), method='DELETE')
    return request.urlopen(req)

basepath = sys.argv[1]

create_response = create_book(basepath)
assert create_response.getcode() == 200
create_response = json.load(create_response)
assert create_response['author'] == 'Jules Verne'
assert create_response['title'] == 'Twenty Thousand Leagues Under the Seas: A World Tour Underwater'
assert 'id' in create_response

get_response = get_books(basepath)
assert get_response.getcode() == 200
get_response = json.load(get_response)
new_book = next((book for book in get_response if book['id'] == create_response['id']), None)
assert new_book['author'] == 'Jules Verne'
assert new_book['title'] == 'Twenty Thousand Leagues Under the Seas: A World Tour Underwater'

delete_response = delete_book(basepath, new_book['id'])
assert delete_response.getcode() == 200
