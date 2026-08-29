# 📦 Sora Library Package

Template pustaka (*library package*) modular untuk bahasa pemrograman Sorayunara (`.sora`).

## Struktur Proyek

```text
├── sorayunara.toml   # Metadata paket & dependensi
├── src/
│   └── lib.sora      # Kode pustaka utama (.sora)
├── tests/
│   └── lib_test.sora # Unit test paket
└── README.md
```

## Cara Menjalankan & Mempublikasikan

```bash
# Menjalankan test
sorayunara run tests/lib_test.sora

# Mempublikasikan ke GitHub Packages (GHCR / OCI)
sorayunara package publish --registry ghcr.io/<username>/sora-math:0.1.0
```
