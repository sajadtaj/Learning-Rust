<div dir='rtl'>

* **Ownership در Rust چیست و چرا اهمیت دارد؟**
  * Ownership مجموعه‌ای از قوانین است که  **نحوه مدیریت حافظه توسط برنامه Rust را کنترل می‌کند** .
  * همه برنامه‌ها باید نحوه استفاده از حافظه کامپیوتر را در حین اجرا مدیریت کنند.
  * برخی زبان‌ها از Garbage Collection استفاده می‌کنند که به طور منظم به دنبال حافظه غیرقابل استفاده می‌گردد؛ در زبان‌های دیگر، برنامه‌نویس باید صراحتاً حافظه را اختصاص داده و آزاد کند.
  * Rust از رویکرد سومی استفاده می‌کند: حافظه از طریق سیستمی از Ownership با مجموعه‌ای از قوانین که کامپایلر آن‌ها را بررسی می‌کند، مدیریت می‌شود.
  * اگر هر یک از قوانین نقض شود، برنامه کامپایل نخواهد شد.
  * هیچ یک از ویژگی‌های Ownership باعث کند شدن برنامه در حین اجرا نمی‌شود.
  * درک Ownership پایه‌ای محکم برای درک ویژگی‌هایی است که Rust را منحصر به فرد می‌کنند. هدف اصلی Ownership مدیریت داده‌های روی heap است.
* **Stack و Heap در حافظه چه تفاوتی دارند و چرا در Rust مطرح می‌شوند؟**
  * Stack و Heap هر دو بخش‌هایی از حافظه هستند که کد شما در زمان اجرا می‌تواند از آن‌ها استفاده کند، اما ساختار آن‌ها متفاوت است.
  * Stack مقادیر را به ترتیبی که دریافت می‌کند ذخیره کرده و به ترتیب معکوس آن‌ها را حذف می‌کند (آخرین ورودی، اولین خروجی یا LIFO). افزودن داده‌ها به Stack را "pushing onto the stack" و حذف داده‌ها را "popping off the stack" می‌نامند.
  * **تمام داده‌های ذخیره شده روی Stack باید اندازه معلوم و ثابتی داشته باشند** . داده‌هایی با اندازه نامعلوم در زمان کامپایل یا اندازه‌ای که ممکن است تغییر کند باید به جای آن روی Heap ذخیره شوند.
  * Heap کمتر سازمان‌یافته است: وقتی داده‌ها را روی Heap قرار می‌دهید، مقدار مشخصی فضا درخواست می‌کنید. تخصیص‌دهنده حافظه یک فضای خالی بزرگ به اندازه کافی در Heap پیدا می‌کند، آن را به عنوان استفاده شده علامت‌گذاری می‌کند و **یک اشاره‌گر (pointer)** که آدرس آن مکان است را برمی‌گرداند. این فرآیند "allocating on the heap" نامیده می‌شود.
  * از آنجایی که اشاره‌گر به Heap اندازه معلوم و ثابتی دارد، می‌توانید اشاره‌گر را روی Stack ذخیره کنید، اما وقتی داده‌های واقعی را می‌خواهید،  **باید اشاره‌گر را دنبال کنید** .
  * Push کردن روی Stack سریع‌تر از اختصاص دادن روی Heap است، زیرا تخصیص‌دهنده هرگز نیازی به جستجو برای مکانی برای ذخیره داده‌های جدید ندارد؛ آن مکان همیشه در بالای Stack است.
  * دسترسی به داده‌ها در Heap کندتر از دسترسی به داده‌ها روی Stack است، زیرا باید اشاره‌گر را دنبال کنید تا به آنجا برسید. پردازنده‌های معاصر اگر کمتر در حافظه جابه‌جا شوند، سریع‌تر هستند.
* **قوانین اصلی Ownership در Rust کدامند؟**
  * هر مقدار در Rust **یک owner** دارد.
  * در هر زمان فقط **یک owner** می‌تواند وجود داشته باشد.
  * وقتی owner از scope خارج می‌شود،  **مقدار drop خواهد شد** .
* **Rust حافظه را برای نوع داده `String` در مقایسه با string literalها چگونه مدیریت می‌کند؟**
  * **String literalها** مقادیر ثابتی هستند که محتوای آن‌ها در زمان کامپایل مشخص است، بنابراین متن مستقیماً در فایل اجرایی نهایی کدگذاری می‌شود. به همین دلیل string literalها سریع و کارآمد هستند، اما  **غیرقابل تغییر (immutable) هستند** . آن‌ها اندازه ثابتی دارند و می‌توانند روی Stack ذخیره شوند.
  * **نوع داده `String`** برای پشتیبانی از متن قابل تغییر و رشد، به **اختصاص مقداری حافظه روی Heap** نیاز دارد که اندازه آن در زمان کامپایل نامعلوم است.
  * وقتی `String::from` را فراخوانی می‌کنیم، حافظه مورد نیاز از تخصیص‌دهنده حافظه در زمان اجرا درخواست می‌شود (allocate).
  * برخلاف زبان‌هایی بدون Garbage Collector که در آن‌ها مسئولیت آزادسازی حافظه با برنامه‌نویس است، در Rust **حافظه به طور خودکار بازگردانده می‌شود** وقتی متغیری که صاحب آن است از scope خارج می‌شود. Rust تابعی خاص به نام `drop` را به طور خودکار در این نقطه (پایان scope) فراخوانی می‌کند تا حافظه را آزاد کند.
* **وقتی یک متغیر `String` را به متغیر دیگری اختصاص می‌دهید (مثلاً `let s2 = s1;`) در Rust چه اتفاقی می‌افتد؟**
  * وقتی `s1` را به `s2` اختصاص می‌دهیم (`let s2 = s1;`)،  **داده‌های `String` کپی می‌شوند** ؛ این به این معنی است که  **اشاره‌گر، طول و ظرفیت که روی Stack قرار دارند، کپی می‌شوند** .
  * **داده‌های روی Heap که اشاره‌گر به آن‌ها اشاره می‌کند، کپی نمی‌شوند** . اگر Rust داده‌های Heap را نیز کپی می‌کرد، این عملیات می‌توانست از نظر عملکرد در زمان اجرا بسیار پرهزینه باشد، به خصوص اگر داده‌های روی Heap بزرگ باشند.
  * بعد از خط `let s2 = s1;`،  **Rust متغیر `s1` را دیگر معتبر نمی‌داند** . این **"move"** نامیده می‌شود.
  * این کار مشکل **double free** (آزادسازی حافظه یکسان دو بار) را حل می‌کند، که می‌تواند منجر به خراب شدن حافظه شود. با معتبر نبودن `s1`، تنها `s2` وقتی از scope خارج می‌شود حافظه را آزاد می‌کند.
  * **Rust هرگز به طور خودکار کپی‌های "عمیق" (deep copies) از داده‌های شما ایجاد نمی‌کند** . بنابراین، هر کپی خودکار را می‌توان از نظر عملکرد در زمان اجرا کم‌هزینه فرض کرد.
* **چگونه می‌توانید داده‌های `String` روی Heap را به صراحت کپی کنید؟**
  * اگر می‌خواهید داده‌های Heap یک `String` را  **به طور عمیق کپی کنید** ، می‌توانید از متد رایج `clone` استفاده کنید.
  * فراخوانی `s1.clone()` به طور صریح رفتار کپی کردن داده‌های Heap را نیز انجام می‌دهد.
  * وقتی فراخوانی `clone` را می‌بینید، می‌دانید که کدی در حال اجرا است که  **ممکن است پرهزینه باشد** .
* **چرا انواع داده ساده (مانند اعداد صحیح) در هنگام اختصاص دادن رفتار متفاوتی نسبت به `String` دارند؟**
  * انواع داده‌ای مانند اعداد صحیح که در زمان کامپایل اندازه معلوم و ثابتی دارند،  **به طور کامل روی Stack ذخیره می‌شوند** .
  * کپی کردن مقادیر واقعی آن‌ها سریع است، بنابراین دلیلی وجود ندارد که بخواهیم `x` را بعد از ایجاد `y` نامعتبر کنیم.
  * در این حالت، تفاوتی بین کپی عمیق و کپی سطحی وجود ندارد.
  * Rust یک annotation ویژه به نام trait به نام `Copy` دارد که می‌توان آن را روی انواع داده‌ای که روی Stack ذخیره می‌شوند، قرار داد.
  * اگر یک نوع داده `Copy` trait را پیاده‌سازی کند، متغیرهایی که از آن استفاده می‌کنند **move نمی‌شوند، بلکه به طور trivilly کپی می‌شوند** و بنابراین پس از اختصاص به متغیر دیگر همچنان معتبر باقی می‌مانند.
  * Rust به شما اجازه نمی‌دهد یک نوع داده را با `Copy` annotate کنید، اگر خود آن نوع یا بخشی از آن، `Drop` trait را پیاده‌سازی کرده باشد.
  * انواع داده‌ای که `Copy` trait را پیاده‌سازی می‌کنند شامل تمام انواع اعداد صحیح، نوع Boolean، تمام انواع اعداد اعشاری، نوع character و Tupleهایی که فقط شامل انواع داده‌ای هستند که `Copy` را پیاده‌سازی می‌کنند، هستند.
* **ارسال مقادیر به توابع چگونه بر Ownership تأثیر می‌گذارد؟**
  * مکانیک ارسال یک مقدار به تابع  **مشابه مکانیک اختصاص دادن یک مقدار به یک متغیر است** .
  * ارسال یک متغیر به یک تابع  **باعث move یا copy شدن آن می‌شود** .
  * اگر مقداری که به تابع ارسال می‌شود از نوعی باشد که `Copy` trait را پیاده‌سازی نمی‌کند (مانند `String`)، Ownership آن **به پارامتر تابع منتقل می‌شود** و متغیر اصلی پس از فراخوانی تابع دیگر معتبر نخواهد بود. وقتی تابع تمام می‌شود، آن متغیر در scope تابع از scope خارج شده و `drop` فراخوانی می‌شود و حافظه آن آزاد می‌گردد.
  * اگر مقداری که به تابع ارسال می‌شود از نوعی باشد که `Copy` trait را پیاده‌سازی می‌کند (مانند `i32`)، آن مقدار **کپی می‌شود** و Ownership منتقل نمی‌شود. متغیر اصلی  **بعد از فراخوانی تابع همچنان معتبر باقی می‌ماند** .

</dev>