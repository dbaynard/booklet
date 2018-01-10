---
title:  Booklet  
author: David Baynard  
date:   10 Jan 2018  
fontfamily:   libertine
abstract: |  
    Reorder PDF pages to form booklet
...

Reorder pages of a pdf for simple booklet printing.

An experiment with [lopdf](https://docs.rs/lopdf/0.14.1/lopdf/) to solve a relatively annoying problem.

Feedback is welcome, on any aspect.

Note: this only corresponds to a single printing strategy.

# What it does

Some printing software can’t reorganise pdfs for booklet form.
This executable (and library) can help.

For example, take an 11 page A4 pdf.
The pages are:

    1 2 3 4 5 6 7 8 9 10 11

When printing as a booklet they should group as follows:

    1 | 2 3 | 4 5 | 6 7 | 8 9 | 10 11 | Blank

On each sheet, then, should be

    6 7 | 8 5
    4 9 | 10 3
    2 11 | Blank 1

and if printed using short edge duplex, this should be the result.

This program therefore rearranges the pages into

    6 7 8 5 4 9 10 3 2 11 Blank 1

generating `Blank` by deleting the contents from page `1`.

# Use

## Install

Clone, then

    > cargo install

Make sure `~/.cargo/bin` is in `$PATH`.

## Run

    > pdfbooklet _infile_ _outfile_

Take care, this will probably clobber _outfile_.

Also, running without arguments will use `stdout` (or `stdin` and `stdout`).

For help,

    > pdfbooklet --help

## Issues

Use the issue tracker.
Submit any files that do not work.

# Future priorities

Priorities are

- [ ] Produce other page patterns for other printers
- [ ] Logging
- [ ] Producing a guide pdf for printer testing
- [ ] 2-up (so the resulting pdf can be printed directly)
