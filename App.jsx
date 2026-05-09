```react
import React, { useState, useEffect, useRef, useMemo } from 'react';
import { 
  ChevronRight, 
  Cpu, 
  Database, 
  ShieldCheck, 
  Zap, 
  Layers, 
  ArrowRight, 
  Terminal as TerminalIcon, 
  Code2, 
  Globe, 
  Lock, 
  Activity,
  Github,
  FileCode,
  BookOpen,
  Check,
  X,
  Search,
  Upload,
  BarChart3,
  HardDrive,
  Network
} from 'lucide-react';
import { motion, AnimatePresence, useScroll, useTransform } from 'framer-motion';

// --- Konfigurasi & Konstanta ---
const LOGO_URL = "https://lh3.googleusercontent.com/d/1BXSeBBtM3BnqPO0YTUO8-VzCRZuhkcdo";
const GITHUB_URL = "https://github.com/zenipara/QRD-SDK";

// --- Utility Components ---

const SectionTitle = ({ title, subtitle, align = "left" }) => (
  <div className={`mb-16 ${align === "center" ? "text-center" : ""}`}>
    <h2 className="text-3xl md:text-4xl font-bold text-white mb-4 tracking-tight">{title}</h2>
    <p className="text-zinc-500 font-mono text-sm max-w-2xl leading-relaxed">{subtitle}</p>
  </div>
);

// --- 1. Binary Stream Background ---
const BinaryBackground = () => {
  return (
    <div className="absolute inset-0 overflow-hidden pointer-events-none opacity-20 z-0">
      <div className="absolute inset-0 bg-[#050505]" />
      <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:40px_40px]" />
      <div className="flex justify-around w-full h-full">
        {[...Array(12)].map((_, i) => (
          <motion.div
            key={i}
            initial={{ y: -100, opacity: 0 }}
            animate={{ y: ['0%', '100%'], opacity: [0, 1, 0] }}
            transition={{ 
              duration: Math.random() * 10 + 10, 
              repeat: Infinity, 
              ease: "linear",
              delay: Math.random() * 10
            }}
            className="text-[10px] font-mono text-cyan-500/30 whitespace-pre"
          >
            {Array(50).fill(0).map(() => Math.round(Math.random())).join('\n')}
          </motion.div>
        ))}
      </div>
    </div>
  );
};

// --- 2. Interactive Architecture Diagram ---
const ArchitectureDiagram = () => {
  const steps = [
    { name: "Raw Ingest", icon: <Database size={18} /> },
    { name: "Encoding", icon: <Code2 size={18} /> },
    { name: "Compression", icon: <Zap size={18} /> },
    { name: "ECC Wiring", icon: <ShieldCheck size={18} /> },
    { name: "Encryption", icon: <Lock size={18} /> },
    { name: "QRD Blob", icon: <HardDrive size={18} /> }
  ];

  return (
    <div className="w-full py-20 relative overflow-hidden bg-zinc-900/20 border border-white/5 rounded-3xl">
      <div className="max-w-5xl mx-auto px-6">
        <div className="flex flex-col md:flex-row items-center justify-between gap-8 relative">
          <div className="absolute top-1/2 left-0 w-full h-[2px] bg-zinc-800 -translate-y-1/2 hidden md:block" />
          
          {steps.map((step, i) => (
            <motion.div 
              key={i}
              initial={{ opacity: 0, scale: 0.8 }}
              whileInView={{ opacity: 1, scale: 1 }}
              viewport={{ once: true }}
              className="relative z-10 flex flex-col items-center group w-full md:w-auto"
            >
              <div className="w-16 h-16 rounded-2xl bg-black border border-white/10 flex items-center justify-center mb-4 group-hover:border-cyan-500/50 group-hover:shadow-[0_0_30px_rgba(6,182,212,0.15)] transition-all duration-500">
                <div className="text-zinc-500 group-hover:text-cyan-400 transition-colors">
                  {step.icon}
                </div>
              </div>
              <span className="text-[10px] font-mono font-bold text-zinc-400 uppercase tracking-widest text-center group-hover:text-white transition-colors">
                {step.name}
              </span>

              {i < steps.length - 1 && (
                <div className="md:hidden h-12 flex items-center justify-center">
                  <div className="w-[1px] h-full bg-zinc-800" />
                </div>
              )}

              {i < steps.length - 1 && (
                <motion.div
                  animate={{ left: ['0%', '100%'] }}
                  transition={{ duration: 2, repeat: Infinity, ease: "linear", delay: i * 0.3 }}
                  className="absolute top-8 left-full w-2 h-2 bg-cyan-400 rounded-full blur-[2px] hidden md:block z-0"
                />
              )}
            </motion.div>
          ))}
        </div>
        
        <div className="mt-16 p-6 rounded-xl bg-black/40 border border-white/5">
          <p className="text-zinc-500 font-mono text-xs leading-relaxed">
            <span className="text-emerald-500">// Analisis Pipeline:</span> QRD menggunakan pendekatan "streaming buffer transition". Data dikompresi dalam blok-blok kecil (chunks) sebelum diaplikasikan Reed-Solomon ECC untuk integritas data maksimal.
          </p>
        </div>
      </div>
    </div>
  );
};

// --- 3. Live Benchmark Terminal ---
const BenchmarkTerminal = () => {
  const [logs, setLogs] = useState([]);
  const [isRunning, setIsRunning] = useState(false);
  
  const runTest = () => {
    if (isRunning) return;
    setIsRunning(true);
    setLogs([]);
    
    const sequence = [
      "> Inisialisasi engine QRD v0.2.0...",
      "> Alokasi buffer SIMD [Instruksi AVX-512 terdeteksi]",
      "> Memuat dataset: sensor_network_2026.csv",
      "> [1/4] Delta-encoding kolom sensor...",
      "> BERHASIL: 2.8 GB/s throughput",
      "> [2/4] Siklus kompresi ZSTD-19...",
      "> Rasio: 15.1x reduksi",
      "> [3/4] Menghasilkan checksum ECC...",
      "> [4/4] Enkripsi dengan AES-256-GCM...",
      "> Paket QRD siap: 6.84 GB",
      "> --- BENCHMARK SELESAI ---"
    ];

    sequence.forEach((line, i) => {
      setTimeout(() => {
        setLogs(prev => [...prev, line]);
        if (i === sequence.length - 1) setIsRunning(false);
      }, i * 500);
    });
  };

  return (
    <div className="bg-[#0c0c0c] border border-white/10 rounded-2xl overflow-hidden shadow-2xl">
      <div className="flex items-center justify-between px-5 py-3 border-b border-white/5 bg-zinc-900/50">
        <div className="flex gap-2">
          <div className="w-3 h-3 rounded-full bg-red-500/20 border border-red-500/40" />
          <div className="w-3 h-3 rounded-full bg-yellow-500/20 border border-yellow-500/40" />
          <div className="w-3 h-3 rounded-full bg-green-500/20 border border-green-500/40" />
        </div>
        <span className="text-[10px] font-mono text-zinc-500 tracking-widest uppercase">qrd-bench --profile=nafal_2026</span>
      </div>
      <div className="p-6 h-[320px] font-mono text-sm overflow-y-auto space-y-2">
        <AnimatePresence>
          {logs.map((log, i) => (
            <motion.div 
              key={i} 
              initial={{ opacity: 0, x: -10 }} 
              animate={{ opacity: 1, x: 0 }}
              className={`${log.includes('BERHASIL') ? 'text-cyan-400' : log.includes('SELESAI') ? 'text-emerald-400 font-bold' : 'text-zinc-400'}`}
            >
              {log}
            </motion.div>
          ))}
        </AnimatePresence>
        {!isRunning && logs.length === 0 && (
          <div className="h-full flex items-center justify-center flex-col gap-4">
            <BarChart3 className="text-zinc-700" size={48} />
            <button 
              onClick={runTest}
              className="px-6 py-2 bg-white text-black text-xs font-bold rounded hover:bg-zinc-200 transition-colors uppercase tracking-widest"
            >
              Jalankan Live Benchmark
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

// --- Main App Component ---
const App = () => {
  const [scrolled, setScrolled] = useState(false);
  const { scrollYProgress } = useScroll();
  const scaleX = useTransform(scrollYProgress, [0, 1], [0, 1]);

  useEffect(() => {
    const handleScroll = () => setScrolled(window.scrollY > 50);
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  return (
    <div className="min-h-screen bg-[#050505] text-zinc-300 selection:bg-cyan-500/30 selection:text-white font-sans overflow-x-hidden">
      
      {/* Scroll Progress */}
      <motion.div className="fixed top-0 left-0 right-0 h-[2px] bg-cyan-500 z-[100] origin-left" style={{ scaleX }} />

      {/* Navbar */}
      <nav className={`fixed top-0 w-full z-[90] transition-all duration-500 border-b ${scrolled ? 'bg-black/80 backdrop-blur-xl border-white/10 py-4' : 'bg-transparent border-transparent py-6'}`}>
        <div className="max-w-7xl mx-auto px-6 flex items-center justify-between">
          <div className="flex items-center gap-3 group cursor-pointer">
            <img src={LOGO_URL} alt="QRD" className="w-8 h-8 group-hover:rotate-12 transition-transform duration-500" />
            <span className="font-mono text-xl font-bold tracking-tighter text-white">QRD</span>
          </div>
          <div className="hidden lg:flex items-center gap-10 text-[11px] font-bold uppercase tracking-[0.2em] text-zinc-500">
            <a href="#arch" className="hover:text-cyan-400 transition-colors">Architecture</a>
            <a href="#bench" className="hover:text-cyan-400 transition-colors">Benchmarks</a>
            <a href={GITHUB_URL} target="_blank" className="hover:text-white flex items-center gap-2"><Github size={14}/> Github</a>
          </div>
          <button className="bg-white text-black px-6 py-2 rounded text-[11px] font-bold uppercase tracking-widest hover:bg-cyan-400 transition-all duration-300">
            Mulai Sekarang
          </button>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="relative pt-44 pb-32 px-6 min-h-screen flex flex-col justify-center border-b border-white/5 overflow-hidden">
        <BinaryBackground />
        
        <div className="max-w-7xl mx-auto relative z-10">
          <motion.div 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="flex flex-wrap gap-4 mb-10"
          >
            <span className="px-3 py-1 bg-cyan-500/10 border border-cyan-500/30 text-cyan-400 text-[10px] font-mono font-bold tracking-widest uppercase rounded-full">PRODUCTION 2026</span>
            <span className="px-3 py-1 bg-white/5 border border-white/10 text-white/50 text-[10px] font-mono font-bold tracking-widest uppercase rounded-full flex items-center gap-2">
              <span className="w-1.5 h-1.5 bg-emerald-500 rounded-full animate-pulse" /> Edge Stable
            </span>
          </motion.div>

          <motion.h1 
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="text-6xl md:text-[100px] font-black text-white tracking-tighter leading-[0.9] mb-8"
          >
            The Edge-Native<br />
            <span className="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-blue-500 to-indigo-500">Binary Engine.</span>
          </motion.h1>

          <motion.p 
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
            className="text-lg md:text-2xl text-zinc-500 max-w-3xl font-mono leading-relaxed mb-16"
          >
            Infrastruktur penyimpanan biner generasi berikutnya oleh Nafal Faturizki. Streaming-first dan dioptimalkan secara ekstrem untuk sistem edge 2026.
          </motion.p>

          <motion.div 
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
            className="flex flex-wrap gap-6 items-center"
          >
            <button className="flex items-center gap-3 bg-white text-black px-10 py-5 rounded-xl font-bold text-lg hover:bg-cyan-400 shadow-[0_0_40px_rgba(34,211,238,0.2)] transition-all group">
              Mulai Eksplorasi <ArrowRight className="group-hover:translate-x-1 transition-transform" />
            </button>
            <a href={GITHUB_URL} target="_blank" className="flex items-center gap-3 bg-zinc-900 border border-white/10 text-white px-10 py-5 rounded-xl font-bold text-lg hover:bg-zinc-800 transition-all">
              <Github size={24} /> Lihat Kode Sumber
            </a>
          </motion.div>
        </div>
      </section>

      {/* Architecture & Benchmarks (Visual-only representation) */}
      <div className="max-w-7xl mx-auto px-6 py-32 space-y-40">
        <section id="arch">
          <SectionTitle 
            title="Arsitektur Engine"
            subtitle="Pipeline transformasi data linier yang dirancang untuk skalabilitas infrastruktur modern."
          />
          <ArchitectureDiagram />
        </section>

        <section id="bench" className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
          <div>
            <SectionTitle 
              title="Performa Tanpa Kompromi."
              subtitle="QRD mengungguli format biner tradisional dengan optimasi hardware Nafal 2026."
            />
            <div className="grid grid-cols-2 gap-6 font-mono text-[10px] uppercase tracking-widest text-zinc-500">
               <div className="p-4 bg-zinc-900/40 border border-white/5 rounded-xl">
                  <div className="text-white text-xl font-bold mb-1">2.8 GB/s</div>
                  Ingest Rate
               </div>
               <div className="p-4 bg-zinc-900/40 border border-white/5 rounded-xl">
                  <div className="text-white text-xl font-bold mb-1">{'<'}2ms</div>
                  Latency Jitter
               </div>
            </div>
          </div>
          <BenchmarkTerminal />
        </section>
      </div>

      {/* Footer */}
      <footer className="py-20 border-t border-white/5 bg-[#030303] relative z-10">
        <div className="max-w-7xl mx-auto px-6 flex flex-col md:flex-row justify-between items-start gap-12">
          <div className="max-w-xs space-y-6">
            <div className="flex items-center gap-3">
              <img src={LOGO_URL} alt="QRD" className="w-6 h-6 grayscale opacity-50" />
              <span className="font-mono text-lg font-bold text-white tracking-tighter uppercase">QRD INFRA</span>
            </div>
            <p className="text-zinc-600 text-[10px] font-mono leading-relaxed uppercase tracking-widest">
              Teknologi infrastruktur biner yang presisi. Dibangun oleh Nafal Faturizki untuk kebutuhan sistem sistem kritis masa depan.
            </p>
            <div className="flex gap-4">
              <a href={GITHUB_URL} target="_blank" className="text-zinc-500 hover:text-white transition-colors"><Github size={20} /></a>
              <a href="#" className="text-zinc-500 hover:text-white transition-colors"><FileCode size={20} /></a>
            </div>
          </div>
          
          <div className="grid grid-cols-2 md:grid-cols-3 gap-16 text-[10px] font-mono font-bold tracking-[0.2em] uppercase">
             <div className="space-y-6">
                <span className="text-zinc-500">Teknologi</span>
                <ul className="space-y-4 text-zinc-400">
                  <li><a href="#" className="hover:text-cyan-400">Spesifikasi v2.0</a></li>
                  <li><a href="#" className="hover:text-cyan-400">Benchmarking</a></li>
                  <li><a href="#" className="hover:text-cyan-400">Audit Keamanan</a></li>
                </ul>
             </div>
             <div className="space-y-6">
                <span className="text-zinc-500">Ekosistem</span>
                <ul className="space-y-4 text-zinc-400">
                  <li><a href="#" className="hover:text-cyan-400">Rust Core</a></li>
                  <li><a href="#" className="hover:text-cyan-400">Node.js Bindings</a></li>
                  <li><a href="#" className="hover:text-cyan-400">WASM Engine</a></li>
                </ul>
             </div>
             <div className="space-y-6">
                <span className="text-zinc-500">Legal</span>
                <ul className="space-y-4 text-zinc-400">
                  <li><a href="#" className="hover:text-cyan-400">Lisensi MIT</a></li>
                  <li><a href="#" className="hover:text-cyan-400">Privasi Data</a></li>
                </ul>
             </div>
          </div>
        </div>
        
        <div className="max-w-7xl mx-auto px-6 mt-20 pt-10 border-t border-white/5 flex flex-col md:flex-row justify-between items-center gap-4 text-zinc-600 text-[10px] font-mono tracking-widest uppercase">
           <span className="text-zinc-400">© 2026 Nafal Faturizki — Semua Hak Dilindungi</span>
           <div className="flex gap-8">
              <span>Sistem Status: <span className="text-emerald-500">Normal</span></span>
              <a href={GITHUB_URL} className="hover:text-white">v0.2.0-STABLE</a>
           </div>
        </div>
      </footer>

      {/* Aesthetic Grain Overlay */}
      <div className="fixed inset-0 pointer-events-none z-[200] opacity-[0.02] mix-blend-overlay">
        <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg" className="w-full h-full">
          <filter id="noiseFilter">
            <feTurbulence type="fractalNoise" baseFrequency="0.65" numOctaves="3" stitchTiles="stitch" />
          </filter>
          <rect width="100%" height="100%" filter="url(#noiseFilter)" />
        </svg>
      </div>
    </div>
  );
};

export default App;

```
