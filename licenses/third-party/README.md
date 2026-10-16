# Supplemental third-party notices

`packages.json` maps exact locked package versions to notice snapshots in
`sources.json`. The latter records pinned upstream URLs. Astryx and StyleX use
their release-tag license; Rust packages use the revision recorded in their
published `.cargo_vcs_info.json`. MIT and MPL standard texts come from SPDX.

The objc2 upstream repository ships a license explanation and links to the
standard terms rather than including standalone license texts. Both its original
explanation and the standard MIT terms are retained. SPDX placeholders do not
identify a copyright owner; no attribution has been invented.

Release packaging includes installed dependency notices and these supplements,
refuses missing notice texts, and attaches unmodified registry source directories
for MPL-covered crates as `MPL_SOURCES.zip`. Their upstream licenses apply;
the proprietary application license does not restrict these sources.
