# LocalAI Studio 🇮🇷

## پلتفرم هوشمند اجرای مدل‌های هوش مصنوعی به‌صورت محلی

LocalAI Studio یک نرم‌افزار دسکتاپ فارسی برای کشف، نصب، مدیریت، اجرا و ارزیابی مدل‌های هوش مصنوعی روی کامپیوتر شخصی است.

### ایده اصلی

برنامه ابتدا سخت‌افزار سیستم را بررسی می‌کند و سپس بر اساس CPU، RAM، GPU، VRAM، فضای دیسک و قابلیت‌های شتاب‌دهنده، مناسب‌ترین مدل و تنظیمات اجرای آن را پیشنهاد می‌دهد.

منبع اصلی کشف مدل‌ها **Hugging Face** است.

## مسیر کاربر

```text
شروع برنامه
    ↓
تشخیص سخت‌افزار
    ↓
تحلیل توان سیستم
    ↓
جستجوی مدل‌ها
    ↓
تحلیل مدل و Variantها
    ↓
بررسی سازگاری
    ↓
انتخاب Quantization
    ↓
انتخاب Runtime
    ↓
ساخت برنامه نصب
    ↓
دانلود
    ↓
اعتبارسنجی
    ↓
اجرای مدل
    ↓
Benchmark
    ↓
تنظیم خودکار
    ↓
چت
```

## ویژگی‌های اصلی

- رابط کاملاً فارسی و RTL
- تشخیص خودکار سخت‌افزار
- تحلیل CPU / RAM / GPU / VRAM
- شناسایی CUDA، ROCm، Vulkan، DirectML و Metal در صورت پشتیبانی سیستم
- جستجوی مدل‌ها از Hugging Face
- تحلیل Format و Quantization
- پیشنهاد هوشمند مدل متناسب با سیستم
- نصب خودکار Runtime
- مدیریت دانلود با Pause / Resume / Retry
- اعتبارسنجی مدل پس از دانلود
- اجرای مدل بدون نیاز به Terminal
- چت محلی
- Benchmark واقعی
- مانیتورینگ CPU / RAM / GPU / VRAM
- API سازگار با OpenAI به‌صورت محلی
- مدیریت چند مدل
- Import مدل‌های محلی
- مرکز عیب‌یابی
- حالت Offline برای اجرای مدل‌های نصب‌شده

## فناوری

| بخش | فناوری |
|---|---|
| Desktop | Tauri 2 |
| رابط کاربری | React + TypeScript |
| طراحی | Tailwind CSS + shadcn/ui |
| مدیریت وضعیت | Zustand |
| هسته | Rust |
| پایگاه داده | SQLite |
| دسترسی DB | SQLx |
| Runtime اولیه | llama.cpp |
| منبع مدل | Hugging Face |
| API | OpenAI-compatible |

## ساختار پروژه

```text
LocalAI-Studio/
├── apps/
│   └── desktop/
├── packages/
├── runtimes/
├── database/
├── tests/
├── scripts/
├── docs/
├── .github/
├── AGENTS.md
├── README.md
└── LICENSE
```

## وضعیت توسعه

پروژه در مرحله **Foundation / معماری اولیه** قرار دارد.

مراحل توسعه:

1. پایه پروژه
2. تشخیص سخت‌افزار
3. اتصال Hugging Face
4. هوش مدل و Recommendation
5. Download Manager
6. Runtime Manager
7. Smart Install
8. Chat
9. API محلی
10. Benchmark
11. Monitoring
12. Runtimeهای پیشرفته
13. قابلیت‌های چندرسانه‌ای
14. Agent هوشمند
15. قابلیت‌های Platform

## اصل مهم

تمام مراحل توسعه، UI، مستندات و پیام‌های برنامه فارسی هستند؛ اما اصطلاحات فنی و شناسه‌های برنامه‌نویسی مانند `GGUF`، `CUDA`، `VRAM`، `llama.cpp` و نام مدل‌ها ترجمه نمی‌شوند.

## مجوز

مجوز پروژه در زمان تثبیت نسخه Production مشخص خواهد شد.
