# Embedded TEMPO2 help

`help.txt` and `help_extra.txt` are byte-for-byte copies from TEMPO2 commit
`fda6ad4a9753ad7dc5d291406f2f310a9161423f`, under the project's
GPL-3.0-or-later terms (see `../LICENSE`). Copyright and source provenance belong
to TEMPO2's authors; the neighboring output sources identify George Hobbs and
Russell Edwards (2006-2009), and help lists its own contributors where present.

SHA-256:

- help.txt: `8201d9edf9571d8bb20812611bdff86eea314f77f10d34138916435cf2efd57d`
- help_extra.txt: `b51c24f0b5a544cea470384566343ced1470a2a00da93a521f2b3dee0f1ae27e`

Pinned `convertxt.c`/Makefile.am embeds these bytes in terminated C arrays.
Rust embeds the text directly; safe byte/string lengths supply the equivalent
termination contract. The command wrapper identifies Rust and its native pin;
plugin mentions in upstream help do not imply an implemented binary plugin ABI.
