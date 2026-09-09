# LocalAI Studio 🇮🇷

## پلتفرم هوشمند اجرای مدل‌های هوش مصنوعی به‌صورت محلی

LocalAI Studio یک نرم‌افزار دسکتاپ فارسی برای کشف، نصب، مدیریت، اجرا و ارزیابی مدل‌های هوش مصنوعی روی کامپیوتر شخصی است.

### ایده اصلی

برنامه ابتدا سخت‌افزار سیستم را بررسی می‌کند و سپس بر اساس CPU، RAM، GPU، VRAM، فضای دیسک و قابلیت‌های شتاب‌دهنده، مناسب‌ترین مدل و تنظیمات اجرای آن را پیشنهاد می‌دهد.

منبع اصلی کشف مدل‌ها **Hugging Face** است.

## مسیر کاربر

```text
شروع برنامه → تشخیص سخت‌افزار → تحلیل توان سیستم → جستجوی مدل‌ها
→ تحلیل Variantها → بررسی سازگاری → انتخاب Quantization
→ انتخاب Runtime → نصب → دانلود → اعتبارسنجی
→ اجرای مدل → Benchmark → تنظیم خودکار → چت
```

## قابلیت‌های فعلی

- رابط فارسی و RTL
- تشخیص CPU / RAM / GPU / VRAM و شتاب‌دهنده‌ها
- جستجوی واقعی Hugging Face
- Recommendation و Smart Install Plan
- دانلود HTTP/HTTPS با SHA-256
- ادامه دانلود از فایل `.part` در سرویس‌هایی که HTTP Range را پشتیبانی می‌کنند
- تشخیص و مدیریت Runtimeهای موجود
- اجرای GGUF با `llama-server`
- Process Manager برای Start / Stop
- Chat با API محلی OpenAI-compatible
- SQLite با WAL و Foreign Keys
- CI خودکار برای Frontend و Rust
- تست‌های Rust در CI
- بسته‌بندی Tauri با آیکون برنامه

## فناوری

| بخش | فناوری |
|---|---|
| Desktop | Tauri 2 |
| رابط کاربری | React + TypeScript |
| مدیریت وضعیت | Zustand |
| هسته | Rust |
| پایگاه داده فعلی | SQLite + rusqlite |
| Runtime اولیه | llama.cpp |
| منبع مدل | Hugging Face Hub API |
| API | OpenAI-compatible local API |

## وضعیت توسعه

**Foundation و Production Hardening در حال تکمیل است.**

مسیر نسخه Production 1.0:

1. Foundation و CI
2. Hardware Intelligence
3. Hugging Face Model Intelligence
4. Recommendation و Quantization
5. Download Manager
6. Runtime Manager
7. Smart Install
8. Model Library
9. Streaming Chat
10. Local OpenAI API
11. Benchmark و Monitoring
12. Recovery و Diagnostics
13. Cross-platform Packaging
14. Multimodal Runtimeها
15. Agent Platform

## اصول مهندسی

- هیچ Performance دقیق و ساختگی به کاربر نمایش داده نمی‌شود؛ مقادیر بر اساس سخت‌افزار، metadata یا benchmark واقعی محاسبه می‌شوند.
- مدل‌ها بدون اعتبارسنجی به حالت Ready منتقل نمی‌شوند.
- API به‌صورت پیش‌فرض فقط روی localhost در دسترس است.
- مسیرهای Runtime و فایل‌های مدل اعتبارسنجی می‌شوند.
- Token، API Key و اطلاعات حساس نباید در Log ثبت شوند.
- خطاها باید قابل تشخیص و قابل بازیابی باشند و Loop بی‌نهایت مجاز نیست.
- تمام مراحل توسعه و پیام‌های کاربر فارسی هستند؛ اصطلاحات فنی مانند `GGUF`، `CUDA`، `VRAM` و `llama.cpp` ترجمه نمی‌شوند.

## مجوز

مجوز پروژه در زمان انتشار Production مشخص خواهد شد.
