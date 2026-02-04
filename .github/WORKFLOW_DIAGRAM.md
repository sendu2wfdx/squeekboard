# GitHub Actions Workflow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    GitHub Actions Workflows                      │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────┐
│                          TRIGGER EVENTS                               │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Push to Branch          Tag Creation           Pull Request         │
│  ───────────────         ─────────────          ────────────         │
│   main/master              v1.44.0               Any branch          │
│       │                       │                       │              │
│       └───────────┬───────────┴──────────┬────────────┘              │
│                   │                      │                           │
└───────────────────┼──────────────────────┼───────────────────────────┘
                    │                      │
                    ▼                      ▼
        ┌───────────────────┐  ┌──────────────────────┐
        │   CI Workflow     │  │ Build & Release      │
        │   (ci.yml)        │  │ (build-and-release)  │
        └───────────────────┘  └──────────────────────┘
                │                       │
                │                       │
                ▼                       ▼
        ┌───────────────────┐  ┌──────────────────────┐
        │ Quick Build Test  │  │  Full Build Process  │
        │                   │  │                      │
        │ • Install deps    │  │  Two Parallel Jobs:  │
        │ • Build with      │  │                      │
        │   meson/ninja     │  │  1. AMD64 Build      │
        │ • Run tests       │  │  2. ARM64 Build      │
        │ • Check artifacts │  │                      │
        └───────────────────┘  └──────────────────────┘
                │                       │
                │                       ├──────────┬─────────────┐
                │                       │          │             │
                ▼                       ▼          ▼             ▼
        ┌───────────────┐      ┌────────────┐ ┌─────────┐ ┌──────────┐
        │  Build Status │      │   AMD64    │ │  ARM64  │ │ Release  │
        │    Updated    │      │   Build    │ │  Build  │ │ Creation │
        └───────────────┘      └────────────┘ └─────────┘ └──────────┘
                                      │            │            │
                                      ▼            ▼            │
                              ┌──────────────────────┐          │
                              │  .deb Generation     │          │
                              ├──────────────────────┤          │
                              │  • AMD64 package     │          │
                              │  • ARM64 package     │          │
                              │  • Build info        │          │
                              └──────────────────────┘          │
                                      │                         │
                                      ▼                         │
                              ┌──────────────────────┐          │
                              │  Upload Artifacts    │          │
                              │  to GitHub Actions   │          │
                              └──────────────────────┘          │
                                      │                         │
                                      │    (If Tag)             │
                                      └────────────────────────►│
                                                                │
                                                                ▼
                                                    ┌───────────────────┐
                                                    │  GitHub Release   │
                                                    ├───────────────────┤
                                                    │  • Download .deb  │
                                                    │  • Release notes  │
                                                    │  • Version tag    │
                                                    └───────────────────┘
```

## Build Process Details

### AMD64 Build (Native)
```
Ubuntu Runner
    │
    ├─► Install Dependencies
    │   └─► Debian packages, Rust toolchain
    │
    ├─► Cache Cargo
    │   └─► registry, git index
    │
    ├─► Build with Meson
    │   └─► meson setup _build
    │       ninja -C _build
    │
    ├─► Run Tests
    │   └─► ninja -C _build test
    │
    ├─► Build .deb Package
    │   └─► debuild -i -us -uc -b
    │
    └─► Upload Artifacts
        └─► *.deb, *.buildinfo, *.changes
```

### ARM64 Build (Emulated)
```
Ubuntu Runner + QEMU
    │
    ├─► Setup QEMU
    │   └─► Enable ARM64 emulation
    │
    ├─► Docker Container (linux/arm64)
    │   │
    │   ├─► Debian Bookworm ARM64
    │   │
    │   ├─► Install Dependencies
    │   │   └─► All build tools
    │   │
    │   ├─► Build with debuild
    │   │   └─► Native ARM64 build
    │   │
    │   └─► Copy packages out
    │
    └─► Upload Artifacts
        └─► *.deb, *.buildinfo, *.changes
```

## Artifact Flow

```
┌─────────────────────────────────────────────────────────┐
│                    Build Complete                        │
└─────────────────────────────────────────────────────────┘
                          │
            ┌─────────────┴─────────────┐
            │                           │
            ▼                           ▼
    ┌───────────────┐          ┌───────────────┐
    │  AMD64 .deb   │          │  ARM64 .deb   │
    │   Package     │          │   Package     │
    └───────────────┘          └───────────────┘
            │                           │
            └─────────────┬─────────────┘
                          │
                          ▼
                ┌──────────────────┐
                │ GitHub Artifacts │
                │   (All Builds)   │
                └──────────────────┘
                          │
                          │ (If Tag Push)
                          ▼
                ┌──────────────────┐
                │ GitHub Release   │
                │  (Tagged Only)   │
                └──────────────────┘
                          │
                          ▼
                ┌──────────────────┐
                │   Users Can      │
                │   Download       │
                └──────────────────┘
```

## Caching Strategy

```
Cargo Registry Cache
    └─► ~/.cargo/registry
        └─► Speeds up dependency download

Cargo Git Cache
    └─► ~/.cargo/git
        └─► Speeds up git dependencies

Cache Key: OS + Cargo.lock hash
    └─► Automatic invalidation on dependency changes
```

## Version Naming

```
Original Version: 1.44.0~alpha0

After Build:
├─► AMD64: 1.44.0~alpha0+github123.abc1234_amd64.deb
│           │               │     │    └─► Architecture
│           │               │     └─────► Git commit hash
│           │               └───────────► Build number
│           └───────────────────────────► Base version
│
└─► ARM64: 1.44.0~alpha0+github-arm64.abc1234_arm64.deb
            │               │            └─► Architecture
            │               └──────────────► Platform identifier
            └──────────────────────────────► Base version
```

## Timeline Example

```
Minute 0:  Developer pushes tag v1.44.0
Minute 1:  GitHub Actions triggered
            ├─► CI Workflow starts
            └─► Build and Release Workflow starts

Minute 2:  AMD64 build starts
           ARM64 build starts

Minute 5:  CI workflow completes ✓
           AMD64 build completes ✓

Minute 20: ARM64 build completes ✓
           (Slower due to QEMU emulation)

Minute 21: Create Release step starts
           ├─► Downloads artifacts
           ├─► Creates GitHub Release
           └─► Uploads .deb files

Minute 22: Release created ✓
           Users can download packages
```
