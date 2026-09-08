# LocalAI Studio — دستورالعمل اصلی توسعه

## زبان پروژه

این پروژه یک نرم‌افزار دسکتاپ فارسی برای اجرای مدل‌های هوش مصنوعی به‌صورت محلی است.

قانون قطعی:
- رابط کاربری: فارسی و راست‌به‌چپ (RTL)
- پیام‌های وضعیت، خطا، راهنما، نصب و اعلان‌ها: فارسی
- مستندات: فارسی
- README: فارسی
- نام‌های فنی، APIها، نام مدل‌ها، نام Runtimeها و شناسه‌های برنامه‌نویسی: انگلیسی و بدون ترجمه
- کامنت‌های مهم کد: فارسی
- کد و نام متغیرها: انگلیسی

## هدف محصول

LocalAI Studio باید مانند یک مرکز مدیریت اجرای مدل‌های هوش مصنوعی محلی عمل کند، اما با یک قابلیت اصلی فراتر از ابزارهای ساده:

> برنامه ابتدا سخت‌افزار را شناسایی می‌کند، توان واقعی سیستم را تحلیل می‌کند، سپس از Hugging Face مدل مناسب، فرمت مناسب، Quantization مناسب و Runtime مناسب را انتخاب و نصب می‌کند.

کاربر نباید برای اجرای معمول مدل مجبور به استفاده از Terminal باشد.

## جریان اصلی محصول

کاربر → اسکن سخت‌افزار → تحلیل قابلیت سیستم → جستجوی مدل‌ها در Hugging Face → تحلیل مدل → بررسی سازگاری → انتخاب Quantization → انتخاب Runtime → ساخت برنامه نصب → دانلود → اعتبارسنجی → اجرای مدل → تست عملکرد → تنظیم خودکار → چت

## معماری

- Desktop: Tauri 2
- Frontend: React + TypeScript
- UI: Tailwind CSS + shadcn/ui
- State: Zustand
- Backend/Core: Rust
- Database: SQLite
- DB access: SQLx
- Runtime اولیه: llama.cpp
- Model Provider اولیه: Hugging Face
- API: OpenAI-compatible local API

معماری باید از ابتدا قابل توسعه برای Ollama، Transformers، vLLM، MLX و Runtimeهای آینده باشد.

## قوانین معماری

1. منطق Runtime داخل UI قرار نگیرد.
2. UI فقط از قراردادهای Core استفاده کند.
3. تمام داده‌های پایدار در SQLite باشند.
4. مسیر مدل‌ها Hardcode نشود.
5. مسیر Binaryهای Runtime ثابت فرض نشود.
6. از HTML scraping برای Hugging Face به‌عنوان روش اصلی استفاده نشود؛ API/Hub رسمی اولویت دارد.
7. هیچ Benchmark ساختگی تولید نشود.
8. اگر عدد عملکرد تخمینی است، صریحاً با برچسب «تخمینی» نمایش داده شود.
9. هیچ Shell command دلخواهی از طرف Agent اجرا نشود.
10. دانلود مدل پس از پایان باید Validate شود.
11. قبل از دانلود فضای دیسک بررسی شود.
12. قبل از Load مدل حافظه و VRAM بررسی شود.
13. در OOM یا خطای Runtime حداکثر ۳ تلاش خودکار با تنظیمات ایمن‌تر انجام شود.
14. API به‌صورت پیش‌فرض فقط روی localhost فعال باشد.
15. Token و API Key هرگز در Log ذخیره نشوند.
16. Prompt کاربر در Log ذخیره نشود.
17. تمام عملیات طولانی Event قابل مشاهده در UI داشته باشند.
18. سیستم باید Offline-first باشد؛ پس از نصب مدل، اجرای مدل و چت نباید به اینترنت وابسته باشند.

## سرویس‌های Core

- HardwareService
- ModelService
- HuggingFaceService
- RecommendationService
- QuantizationService
- RuntimeService
- DownloadService
- BenchmarkService
- ProcessService
- MemoryService
- ChatService
- ApiService
- StorageService
- SecurityService
- DiagnosticsService

## Runtime Interface

```rust
pub trait RuntimeAdapter {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn detect(&self) -> RuntimeDetectionResult;
    fn install(&self) -> RuntimeResult;
    fn update(&self) -> RuntimeResult;
    fn supports_model(&self, model: &ModelDescriptor) -> CompatibilityResult;
    fn build_command(&self, model: &ModelDescriptor, config: &InferenceConfig) -> RuntimeCommand;
    fn launch(&self, command: RuntimeCommand) -> RuntimeResult;
    fn stop(&self, process_id: u32) -> RuntimeResult;
    fn health_check(&self) -> HealthResult;
}
```

## زبان UI

زبان پیش‌فرض فارسی است.

نمونه اصطلاحات:
- Dashboard → داشبورد
- Hardware → سخت‌افزار
- Models → مدل‌ها
- Downloads → دانلودها
- Runtime → موتور اجرا
- Benchmark → ارزیابی عملکرد
- Settings → تنظیمات
- Compatibility → سازگاری
- Recommended → پیشنهادی
- Install → نصب
- Run → اجرا
- Stop → توقف
- Memory → حافظه
- VRAM → حافظه گرافیکی (VRAM)

## صفحات اصلی

1. خوش‌آمدگویی
2. اسکن سخت‌افزار
3. داشبورد
4. مدل‌ها
5. جزئیات مدل
6. دانلودها
7. مدل‌های نصب‌شده
8. موتورهای اجرا
9. چت
10. ارزیابی عملکرد
11. مانیتورینگ
12. API محلی
13. عیب‌یابی
14. تنظیمات

## Smart Install

ورودی:

```text
smart_install({
  task,
  preferences,
  model_id: optional
})
```

خروجی باید شامل مدل، Variant، Runtime، Backend، تنظیمات، حجم دانلود، حافظه موردنیاز و هشدارها باشد.

## Recommendation

وزن پیش‌فرض:

- ۳۰٪ سازگاری سخت‌افزار
- ۲۵٪ عملکرد
- ۲۰٪ کیفیت مدل
- ۱۵٪ تناسب با وظیفه
- ۱۰٪ سازگاری Runtime

این وزن‌ها باید قابل تنظیم باشند.

## Hardware

اطلاعات موردنیاز:

- سیستم‌عامل
- معماری
- CPU، هسته‌ها، فرکانس و Instruction Set
- RAM کل و آزاد
- تمام GPUها
- VRAM
- Driver
- CUDA
- ROCm
- Vulkan
- DirectML
- Metal
- OpenCL
- فضای ذخیره‌سازی

## مدل‌ها

فرمت‌های معماری‌شده:

- GGUF
- Safetensors
- PyTorch
- ONNX
- AWQ
- GPTQ
- EXL2
- MLX

Quantizationهای مهم:

- F32
- F16
- BF16
- Q2
- Q3
- Q4
- Q5
- Q6
- Q8
- Q4_K_M
- Q5_K_M
- Q6_K
- Q8_0
- GPTQ
- AWQ
- EXL2

برای GGUF اطلاعات واقعی فایل و Metadata بررسی شود و فقط به نام فایل اعتماد نشود.

## Database

حداقل جداول:

hardware_profiles, gpus, hardware_benchmarks, models, model_variants, model_files, model_tags, runtimes, runtime_versions, downloads, installed_models, model_benchmarks, inference_sessions, chat_sessions, chat_messages, api_servers, performance_samples, settings

## Eventها

```text
hardware.scan.started
hardware.scan.completed
model.search.started
model.search.completed
download.started
download.progress
download.paused
download.completed
download.failed
runtime.install.started
runtime.install.completed
runtime.install.error
model.loading
model.ready
model.stopped
model.error
generation.started
generation.token
generation.finished
generation.error
benchmark.started
benchmark.progress
benchmark.completed
```

## تست

قبل از اعلام هر مرحله به‌عنوان کامل:

1. Format
2. Lint
3. Typecheck
4. Unit Test
5. Integration Test در صورت وجود
6. Build
7. بررسی خطاهای Runtime

چرخه توسعه:

> پیاده‌سازی → تست → اجرا → تشخیص خطا → اصلاح → بازآزمایی → Refactor → تست نهایی

## دستور مهم برای OpenCode

این پروژه را مرحله‌ای اما بدون نیاز به تأیید کاربر توسعه بده. پس از پایان هر مرحله مستقیماً مرحله بعد را اجرا کن. از Mock برای مسیرهای Production استفاده نکن. فقط در تست‌ها Mock مجاز است.

اگر یک سرویس خارجی در محیط توسعه موقتاً در دسترس نبود، مرز Adapter تمیز ایجاد کن و تست را با Mock انجام بده، اما مسیر Production واقعی باقی بماند.

هدف نهایی نسخه Production است، نه Prototype ظاهری.
