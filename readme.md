# Distributed File Sharing
`This repository is in progress` 
A peer-to-peer distributed file sharing system

# Increments
1. send packets between clients
2. dir request to see all avaliable files
3. request to fetch a file
4. tracker communications to search for query
5. peer sending to tracker the unique words in its file names
6. tracker indexing

# Testing
nc -kl 127.0.0.1 8080

## old
## Definitions
`Tracker`:
- a server that keeps track of one or more `Torrents`
- keeps track of complete and incomplete peer downloads
- recieves HTTP/HTTPS GET requests
- responds with "text/plain" `bencoded` data
- can be scraped to get all `Torrents` the `Tracker` is managing

`Torrent`:
- also known as the `metainfo file`
- a `bencoded` metadata file
- contains information about `pieces`
- can contain pieces belonging to one file or multiple files

`pieces`:
- a file that you want to distribute is split up into pieces
- each piece is usually 512KB
