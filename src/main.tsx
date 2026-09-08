import React from 'react';
import { createRoot } from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import './styles.css';
import { useStudio } from './store';

type Descriptor = {
  id: string;
  name: string;
  author: string;
  task: string;
  downloads?: number;
  likes?: number;
  tags: string[];
  variants: { id: string; format: string; quantization?: string; file_size: number }[];
};

type Plan = {
  model: string;
  variant: string;
  runtime: string;
  backend: string;
  estimated_download_size: number;
  estimated_memory: number;
  compatibility_score: number;
  warnings: string[];
};

type RuntimeInfo = {
  id: string;
  installed: boolean;
  healthy: boolean;
  binary?: string;
  version?: string;
  capabilities: string[];
};

type Message = { role: 'user' | 'assistant'; content: string };

const nav = [
  ['dashboard', 'داشبورد'],
  ['hardware', 'سخت‌افزار'],
  ['models', 'مدل‌ها'],
  ['runtime', 'Runtime'],
  ['chat', 'گفتگو'],
  ['settings', 'تنظیمات'],
] as const;

function App() {
  const { page, setPage, hardware, scan, running } = useStudio();
  React.useEffect(() => void scan(), [scan]);
  const title = nav.find(([id]) => id === page)?.[1] ?? 'LocalAI Studio';
  return (
    <div className="app" dir="rtl">
      <aside>
        <div className="brand">◈ <span>LocalAI Studio</span></div>
        {nav.map(([id, label]) => (
          <button key={id} className={page === id ? 'nav active' : 'nav'} onClick={() => setPage(id)}>{label}</button>
        ))}
      </aside>
      <main>
        <header>
          <div><small>پلتفرم هوشمند اجرای مدل محلی</small><h1>{title}</h1></div>
          <button className="scan" onClick={() => void scan()}>↻ اسکن سخت‌افزار</button>
        </header>
        {page === 'dashboard' && <Dashboard />}
        {page === 'hardware' && <Hardware />}
        {page === 'models' && <Models />}
        {page === 'runtime' && <Runtime />}
        {page === 'chat' && <Chat />}
        {page === 'settings' && <Settings />}
        {running && <div className="notice">مدل فعال: {running}</div>}
      </main>
    </div>
  );
}

function Dashboard() {
  const { hardware, running, setPage } = useStudio();
  return <>
    <section className="hero">
      <div>
        <span className="pill">Offline-first · Local Runtime</span>
        <h2>هوش مصنوعی محلی، متناسب با سیستم شما</h2>
        <p>سخت‌افزار واقعی سیستم شناسایی می‌شود، مدل‌ها از Hugging Face جستجو می‌شوند و اجرای GGUF از طریق llama.cpp انجام می‌شود.</p>
        <button onClick={() => setPage('models')}>کشف مدل‌ها ←</button>
      </div>
      <div className="orb">AI</div>
    </section>
    <section className="panel">
      <h2>وضعیت سیستم</h2>
      <div className="stats">
        <div>CPU<strong>{hardware.cpu}</strong></div>
        <div>RAM<strong>{hardware.ram}</strong></div>
        <div>GPU<strong>{hardware.gpu}</strong></div>
        <div>مدل فعال<strong>{running || 'هیچ مدل فعالی نیست'}</strong></div>
      </div>
    </section>
  </>;
}

function Hardware() {
  const { hardware, scan } = useStudio();
  return <section className="panel">
    <div className="panel-head"><h2>پروفایل سخت‌افزار</h2><button onClick={() => void scan()}>اسکن مجدد</button></div>
    <div className="stats">
      <div>سیستم‌عامل<strong>{hardware.os}</strong></div>
      <div>پردازنده<strong>{hardware.cpu}</strong></div>
      <div>RAM کل / آزاد<strong>{hardware.ram} / {hardware.availableRam}</strong></div>
      <div>Accelerator<strong>{hardware.backend}</strong></div>
    </div>
    <div className="panel"><b>GPU</b><p>{hardware.gpu}</p></div>
    <div className="notice">VRAM فقط وقتی نمایش داده می‌شود که از ابزار واقعی سیستم قابل خواندن باشد؛ مقدار ساختگی تولید نمی‌شود.</div>
  </section>;
}

function Models() {
  const { models, runPath } = useStudio();
  const [query, setQuery] = React.useState('qwen gguf');
  const [remote, setRemote] = React.useState<Descriptor[]>([]);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState('');
  const [plan, setPlan] = React.useState<Plan | null>(null);
  const [localPath, setLocalPath] = React.useState('');

  async function search() {
    setLoading(true); setError('');
    try {
      const result = await invoke<Descriptor[]>('search_models', { query, limit: 12 });
      setRemote(result);
    } catch (e) {
      setRemote([]); setError(String(e));
    } finally { setLoading(false); }
  }

  async function inspect(id: string) {
    setError('');
    try {
      const result = await invoke<Plan>('smart_install_plan', { modelId: id, variantId: 'Q4_K_M' });
      setPlan(result);
    } catch (e) { setError(String(e)); }
  }

  async function runLocal() {
    setError('');
    try { await runPath(localPath.trim()); }
    catch (e) { setError(String(e)); }
  }

  return <section className="panel">
    <div className="panel-head"><div><h2>Model Explorer</h2><p>جستجو از Backend برنامه انجام می‌شود؛ Browser مستقیماً Token یا API خارجی را مدیریت نمی‌کند.</p></div></div>
    <div className="toolbar"><input className="search" value={query} onChange={e => setQuery(e.target.value)} onKeyDown={e => e.key === 'Enter' && void search()} placeholder="Qwen, Llama, coding, GGUF..."/><button onClick={() => void search()}>{loading ? 'در حال جستجو…' : 'جستجو'}</button></div>
    {error && <div className="notice warn">{error}</div>}
    {remote.map(m => <div className="model" key={m.id}><div><b>{m.id}</b><p>{m.task} · دانلود {formatNumber(m.downloads ?? 0)} · پسندیده {formatNumber(m.likes ?? 0)}</p></div><span className="tag">Hugging Face</span></div>)}

    <h3>پیشنهادهای اولیه قابل تحلیل</h3>
    {models.map(m => <div className="model" key={m.id}><div><b>{m.name}</b><p>{m.task} · {m.quant} · {m.size}</p></div><button onClick={() => void inspect(m.id)}>Smart Plan</button></div>)}
    {plan && <div className="plan"><h3>برنامه نصب پیشنهادی</h3><p>{plan.model} / {plan.variant}</p><div className="stats"><div>امتیاز<strong>{plan.compatibility_score}%</strong></div><div>Runtime<strong>{plan.runtime}</strong></div><div>Backend<strong>{plan.backend}</strong></div><div>RAM تخمینی<strong>{bytes(plan.estimated_memory)}</strong></div></div>{plan.warnings.map(w => <div className="notice warn" key={w}>{w}</div>)}</div>}

    <div className="panel">
      <h3>اجرای مدل GGUF محلی</h3>
      <p>مسیر کامل فایل GGUF را وارد کنید. برنامه فقط فایل واقعی را می‌پذیرد و llama-server را روی localhost اجرا می‌کند.</p>
      <div className="toolbar"><input className="search" value={localPath} onChange={e => setLocalPath(e.target.value)} placeholder="C:\\Models\\model.gguf یا /home/user/models/model.gguf"/><button disabled={!localPath.trim()} onClick={() => void runLocal()}>اجرا</button></div>
    </div>
  </section>;
}

function Runtime() {
  const { running, stop } = useStudio();
  const [items, setItems] = React.useState<RuntimeInfo[]>([]);
  const [error, setError] = React.useState('');
  const refresh = React.useCallback(async () => {
    try { setItems(await invoke<RuntimeInfo[]>('runtime_status')); setError(''); }
    catch (e) { setError(String(e)); }
  }, []);
  React.useEffect(() => void refresh(), [refresh]);
  return <section className="panel">
    <div className="panel-head"><h2>Runtime Manager</h2><button onClick={() => void refresh()}>بررسی مجدد</button></div>
    {error && <div className="notice warn">{error}</div>}
    {items.map(r => <div className="runtime" key={r.id}><div><b>{r.id}</b><p>{r.installed ? 'نصب شده' : 'نصب نشده'} · {r.version ?? 'نسخه نامشخص'}<br/>{r.binary ?? 'Binary در PATH پیدا نشد'}</p></div><span className={r.healthy ? 'good' : 'warn'}>{r.healthy ? 'سالم' : 'نیازمند نصب'}</span></div>)}
    {running && <button onClick={() => void stop()}>توقف مدل فعال</button>}
  </section>;
}

function Chat() {
  const { running } = useStudio();
  const [text, setText] = React.useState('');
  const [messages, setMessages] = React.useState<Message[]>([]);
  const [busy, setBusy] = React.useState(false);
  const [error, setError] = React.useState('');

  async function send(e: React.FormEvent) {
    e.preventDefault();
    const content = text.trim();
    if (!content || busy) return;
    const next = [...messages, { role: 'user' as const, content }];
    setMessages(next); setText(''); setBusy(true); setError('');
    try {
      const reply = await invoke<string>('chat_completion', { messages: next });
      setMessages([...next, { role: 'assistant', content: reply }]);
    } catch (err) { setError(String(err)); }
    finally { setBusy(false); }
  }

  return <section className="panel chat">
    <div className="panel-head"><div><h2>گفتگوی محلی</h2><p>{running ? `Runtime فعال برای ${running}` : 'ابتدا یک فایل GGUF را اجرا کنید.'}</p></div></div>
    <div className="messages">{messages.map((m, i) => <div className={m.role === 'user' ? 'user' : 'ai'} key={`${m.role}-${i}`}>{m.content}</div>)}</div>
    {error && <div className="notice warn">{error}</div>}
    <form onSubmit={send}><input value={text} onChange={e => setText(e.target.value)} disabled={!running || busy} placeholder={running ? 'پیام خود را بنویسید…' : 'مدلی اجرا نشده است'}/><button disabled={!running || busy}>{busy ? 'در حال تولید…' : 'ارسال'}</button></form>
  </section>;
}

function Settings() {
  return <section className="panel"><h2>تنظیمات v1.0</h2><div className="settings"><label>API محلی<input value="127.0.0.1:1234" readOnly/></label><label>Runtime اصلی<input value="llama.cpp" readOnly/></label><label>فرمت اجرای مستقیم<input value="GGUF" readOnly/></label></div><div className="notice">سرور Runtime فقط روی localhost راه‌اندازی می‌شود.</div></section>;
}

function formatNumber(n: number) { return new Intl.NumberFormat('fa-IR', { notation: 'compact' }).format(n); }
function bytes(n: number) { return `${(n / 1024 / 1024 / 1024).toFixed(1)} GB`; }

createRoot(document.getElementById('root')!).render(<App />);
