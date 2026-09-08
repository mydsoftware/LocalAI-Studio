import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export type Model = {
  id: string;
  name: string;
  task: string;
  quant: string;
  size: string;
  score: number;
};

type HW = {
  cpu: string;
  ram: string;
  gpu: string;
  backend: string;
  os: string;
  availableRam: string;
};

type State = {
  page: string;
  setPage: (page: string) => void;
  hardware: HW;
  models: Model[];
  running: string;
  scan: () => Promise<void>;
  runPath: (modelPath: string) => Promise<void>;
  stop: () => Promise<void>;
};

const fallback: Model[] = [
  {
    id: 'Qwen/Qwen3-8B-GGUF',
    name: 'Qwen3 8B',
    task: 'گفتگو و استدلال',
    quant: 'Q4_K_M',
    size: '≈ 5 GB',
    score: 0,
  },
  {
    id: 'Qwen/Qwen2.5-Coder-7B-Instruct-GGUF',
    name: 'Qwen2.5 Coder 7B',
    task: 'کدنویسی',
    quant: 'Q4_K_M',
    size: '≈ 4.7 GB',
    score: 0,
  },
];

const initialHardware: HW = {
  cpu: 'در حال شناسایی…',
  ram: 'در حال شناسایی…',
  gpu: 'در حال شناسایی…',
  backend: 'در حال شناسایی…',
  os: '—',
  availableRam: '—',
};

function bytes(value: number) {
  return `${(value / 1024 / 1024 / 1024).toFixed(1)} GB`;
}

export const useStudio = create<State>((set, get) => ({
  page: 'dashboard',
  setPage: (page) => set({ page }),
  hardware: initialHardware,
  models: fallback,
  running: '',
  scan: async () => {
    try {
      const h = await invoke<{
        os: string;
        os_version: string;
        architecture: string;
        cpu: string;
        logical_cores: number;
        ram_bytes: number;
        available_ram_bytes: number;
        accelerators: string[];
        gpus: { name: string; vram_bytes: number }[];
      }>('scan_hardware');
      set({
        hardware: {
          cpu: `${h.cpu} · ${h.logical_cores} هسته`,
          ram: bytes(h.ram_bytes),
          availableRam: bytes(h.available_ram_bytes),
          gpu: h.gpus.length
            ? h.gpus.map((g) => `${g.name}${g.vram_bytes ? ` · ${bytes(g.vram_bytes)}` : ''}`).join('، ')
            : 'GPU قابل اندازه‌گیری شناسایی نشد',
          backend: h.accelerators.join(' / '),
          os: `${h.os} ${h.os_version} · ${h.architecture}`,
        },
      });
    } catch {
      set({
        hardware: {
          ...initialHardware,
          cpu: 'Backend در دسترس نیست',
          ram: '—',
          gpu: '—',
          backend: '—',
        },
      });
    }
  },
  runPath: async (modelPath) => {
    const result = await invoke<{ id: string; pid: number }>('start_model', {
      modelPath,
      config: null,
    });
    set({ running: result.id });
  },
  stop: async () => {
    const modelPath = get().running;
    if (!modelPath) return;
    await invoke('stop_model', { modelPath });
    set({ running: '' });
  },
}));
