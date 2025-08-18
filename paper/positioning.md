# Executable Protocol Specifications (EPS) for Host–GPU APIs

**Idea.** SPEC-Invarianten werden als *compile-time obligations* formuliert und durch
**executable proofs** (compile-fail Tests mit `trybuild`) belegt.

**Research Questions.**
- RQ1: Wie viel einer Fehlermuster-Taxonomie deckt EPS ab? (PSC)
- RQ2: Wo sind Grenzen (dynamische Protokolle, Message-Stabilität)?
- RQ3: Übertragbarkeit (SYCL/CUDA-Graphs Skizzen)
- RQ4: Overhead/Größe im Hot-Path (Criterion + bloat/ASM)

*Notes:* trybuild für compile-fail; Toolchain pinning gegen Flakiness.  
