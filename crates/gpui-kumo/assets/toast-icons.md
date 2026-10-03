# Toast icon provenance

The five `toast-*.svg` assets are exported without path modifications from `@phosphor-icons/react` **2.1.10**, the pinned source fixture dependency. Success uses filled CheckCircle, error filled WarningOctagon, warning filled Warning, info filled Info, and close regular X. The native component supplies size/color; SVG black is a mask color.

Copyright and MIT terms are retained in [toast-icons.LICENSE](toast-icons.LICENSE). Reproduce with `node docs/evidence/toast/browser/extract-icons.cjs` after installing that fixture's pinned dependencies. The extraction script bundles React server rendering into a temporary directory and removes it afterward; no bundled runtime is shipped.
