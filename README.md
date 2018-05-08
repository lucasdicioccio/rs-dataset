dataset : a CLI program to put order in small datasets
======================================================

This tool is a response to a need I encountered when doing statistics "in the
small". I get a number of small datasets, extracts, and excerpts from various
people. Each person uses a somewhat different tool (Excel, SQL database,
hand-crafted scripts etc). I'm not a fond of large ETL solutions. Rather,
following the UNIX philosophy I typically would organize all my datasets for a
given task under a same directory and run a `R` or `Python` session aside for
the data exploration. This tool industrializes the manual work:

- move files under a directory structure with data blobs and descriptions separated
- uses content-addressable storage using a `md5` hash (note: `md5` is a weak hash => use this tool only if you trust the dataset sources)
- has primitive search functions

Longer term I'd like to add:

- merging functions between two `.dataset` hierarchies
- more metadata about mime/types and formats
- dataset preview functions
- export functions for when the number of datasets becomes too large

# Please use alternatives

This tool is meant for handling small amount of datasets. If you need to handle
thousands of dataset in a single exploration. I wanted to make a non-trivial
Rust program to try this promising language, at this point I have not put
enough efforts to recommend anyone using this tool (hence, the lack of 'HOWTO'
section in this README).  At this point I recommend people to leverage
[git-annex](https://git-annex.branchable.com/) to achieve a similar result.
