<html dir="rtl">

**تکلیف:**

هدف این تکلیف، تمرین مفاهیم تعریف و استفاده از Struct ها، نمونه‌سازی (Instantiation)، دسترسی و تغییر فیلدها، استفاده از توابع برای ساخت Struct ها، و همچنین استفاده از میانبرهای `field init shorthand` و `struct update syntax` است.

لطفاً مراحل زیر را در یک برنامه Rust پیاده‌سازی کنید:

1. **تعریف یک Struct:** یک Struct به نام `Book` تعریف کنید که اطلاعات مربوط به یک کتاب را ذخیره کند. این Struct باید شامل فیلدهای زیر باشد:
   * `title`: از نوع `String` (عنوان کتاب).
   * `author`: از نوع `String` (نویسنده کتاب).
   * `isbn`: از نوع `String` (شماره استاندارد بین‌المللی کتاب).
   * `published_year`: از نوع `u16` (سال انتشار).
   * `is_available`: از نوع `bool` (وضعیت موجود بودن کتاب در انبار).
   * **نکته:** برای فیلدهای متنی مانند `title`، `author`، و `isbn` از نوع `String` استفاده کنید تا Struct مالک داده‌های خود باشد.
2. **ایجاد نمونه‌ای از Struct:** در تابع `main`، یک نمونه (instance) از Struct `Book` به نام `book1` ایجاد کنید و مقادیر اولیه را برای فیلدهای آن مشخص نمایید. این نمونه باید **قابل تغییر (mutable)** باشد.
3. **تعریف تابعی برای ایجاد Struct:** یک تابع به نام `create_book` تعریف کنید که عنوان (`title` از نوع `String`)، نویسنده (`author` از نوع `String`)، و سال انتشار (`published_year` از نوع `u16`) را به عنوان ورودی دریافت کند و یک نمونه از Struct `Book` را برگرداند. در این تابع، فیلد `isbn` را به صورت پیش‌فرض یک رشته خالی (`String::from("")`) و فیلد `is_available` را به صورت پیش‌فرض `true` قرار دهید. برای فیلدهای `title`، `author` و `published_year` که نام پارامترهای تابع با نام فیلدهای Struct یکسان است، از **میانبر `field init shorthand`** استفاده کنید.
4. **استفاده از تابع `create_book`:** در تابع `main`، با استفاده از تابع `create_book`، یک نمونه دیگر از Struct `Book` به نام `book2` ایجاد کنید.
5. **تغییر فیلدهای نمونه قابل تغییر:** فیلد `is_available` در نمونه `book1` را به `false` تغییر دهید. همچنین فیلد `isbn` آن را به یک مقدار معتبر (مثلاً `"978-3-16-148410-0"`) تغییر دهید. **به یاد داشته باشید که برای تغییر یک فیلد، کل نمونه Struct باید `mutable` (با استفاده از `let mut`) تعریف شده باشد، نه فقط آن فیلد خاص.**
6. **ایجاد نمونه جدید با استفاده از `struct update syntax`:** یک نمونه جدید از Struct `Book` به نام `book3` ایجاد کنید. از `struct update syntax` برای کپی کردن بیشتر فیلدهای `book2` در `book3` استفاده کنید، اما مقدار فیلد `is_available` در `book3` را به صورت جداگانه `false` تنظیم کنید.
   * **نکته در مورد مالکیت (Ownership):** به خاطر داشته باشید که `struct update syntax` داده‌ها را **انتقال (move)** می‌دهد، نه کپی. فیلدهایی از `book2` که از نوع `String` هستند (مثل `title`, `author`, `isbn`) به `book3` منتقل می‌شوند و پس از آن نمی‌توانید از این فیلدها در `book2` استفاده کنید. اما فیلدهایی مانند `published_year` (که از نوع `u16` است و `Copy` trait را پیاده‌سازی می‌کند) کپی می‌شوند.
7. **دسترسی و نمایش فیلدها (اختیاری اما توصیه می‌شود):** از dot notation برای دسترسی به فیلدهای نمونه‌های `book1`, `book2` و `book3` استفاده کنید و آن‌ها را (مثلاً با استفاده از `println!`) نمایش دهید تا تأیید کنید تغییرات و نمونه‌های جدید به درستی ایجاد شده‌اند.

---

**پاسخ (راه حل تفصیلی همراه با کد):**

در ادامه، راه‌حل این تکلیف به همراه توضیحات گام به گام آورده شده است.

**توضیحات و کد:**

ابتدا فایل `src/main.rs` را باز کنید (اگر از Cargo استفاده می‌کنید) یا کد زیر را در یک فایل `.rs` ذخیره کنید.

1. **تعریف Struct `Book`:**
   ما Struct `Book` را با فیلدهای مشخص شده و انواع داده متناسب تعریف می‌کنیم. برای فیلدهای متنی از `String` استفاده می‌کنیم تا مطمئن شویم Struct مالک این داده‌هاست.

   ```
   struct Book {
       title: String,
       author: String,
       isbn: String,
       published_year: u16,
       is_available: bool,
   }
   ```

   این تعریف یک قالب کلی برای نوع `Book` ایجاد می‌کند.
2. **ایجاد نمونه قابل تغییر `book1`:**
   در تابع `main`، یک نمونه از `Book` به نام `book1` ایجاد می‌کنیم و از کلمه کلیدی `mut` استفاده می‌کنیم تا آن را قابل تغییر سازیم. مقادیر اولیه را برای هر فیلد مشخص می‌کنیم. ترتیب فیلدها در اینجا اهمیتی ندارد.

   ```
   fn main() {
       let mut book1 = Book {
           title: String::from("The Hitchhiker's Guide to the Galaxy"),
           author: String::from("Douglas Adams"),
           isbn: String::from("978-0-345-39180-3"),
           published_year: 1979,
           is_available: true,
       };
       // ... ادامه کد در مراحل بعدی
   }
   ```
3. **تعریف تابع `create_book` با `field init shorthand`:**
   تابعی به نام `create_book` تعریف می‌کنیم که پارامترهای ورودی آن (`email`, `username`) نام مشابه با فیلدهای Struct `Book` دارند. این تابع یک نمونه `Book` برمی‌گرداند. داخل تابع، هنگام ایجاد نمونه `Book`، برای فیلدهایی که نام آن‌ها با نام پارامترهای ورودی یکسان است (`title`, `author`, `published_year`)، فقط نام فیلد را می‌نویسیم (مثلاً `title` به جای `title: title`).

   ```
   fn create_book(title: String, author: String, published_year: u16) -> Book {
       Book {
           title, // استفاده از field init shorthand
           author, // استفاده از field init shorthand
           published_year, // استفاده از field init shorthand
           isbn: String::from(""), // مقدار پیش فرض
           is_available: true, // مقدار پیش فرض
       }
   }
   // ... بقیه کد تابع main
   ```
4. **استفاده از تابع `create_book` برای `book2`:**
   در تابع `main`، تابع `create_book` را فراخوانی می‌کنیم تا نمونه `book2` را بسازیم.

   ```
   fn main() {
       let mut book1 = Book { /* ... */ }; // کد قبلی book1

       let book2 = create_book(
           String::from("The Restaurant at the End of the Universe"),
           String::from("Douglas Adams"),
           1980,
       );
       // ... ادامه کد در مراحل بعدی
   }
   ```
5. **تغییر فیلدهای `book1`:**
   چون `book1` را با `let mut` تعریف کرده‌ایم، می‌توانیم با استفاده از dot notation مقادیر فیلدهای آن را تغییر دهیم.

   ```
   fn main() {
       let mut book1 = Book { /* ... */ }; // کد قبلی book1
       let book2 = create_book(/* ... */); // کد قبلی book2

       // تغییر فیلد is_available در book1
       book1.is_available = false;

       // تغییر فیلد isbn در book1
       book1.isbn = String::from("978-3-16-148410-0");

       // ... ادامه کد در مراحل بعدی
   }
   ```
6. **ایجاد نمونه `book3` با `struct update syntax`:**
   Struct update syntax (`..`) به ما اجازه می‌دهد بیشتر فیلدهای یک نمونه جدید را با کپی کردن مقادیر از یک نمونه موجود دیگر پر کنیم و فقط فیلدهایی را که می‌خواهیم متفاوت باشند، صریحاً مشخص کنیم. `..book2` باید آخرین مورد در لیست فیلدها باشد.

   ```
   fn main() {
       let mut book1 = Book { /* ... */ }; // کد قبلی book1
       let book2 = create_book(/* ... */); // کد قبلی book2
       // تغییرات book1

       let book3 = Book {
           is_available: false, // مشخص کردن فیلد متفاوت
           ..book2 // استفاده از بقیه فیلدها از book2
       };
       // ... ادامه کد در مراحل بعدی
   }
   ```

   **نکته مالکیت:** در اینجا، فیلدهای `title`, `author`, و `isbn` از `book2` به `book3` **منتقل (moved)** می‌شوند. پس از این خط، نمی‌توانید از `book2.title`، `book2.author` یا `book2.isbn` استفاده کنید. اما فیلد `published_year` که نوع `u16` دارد و Copy trait را پیاده‌سازی می‌کند، کپی می‌شود و همچنان قابل دسترسی در `book2` است.
7. **دسترسی و نمایش فیلدها (اختیاری):**
   برای بررسی نتایج، می‌توانیم فیلدهای Struct ها را چاپ کنیم.

   ```
   fn main() {
       let mut book1 = Book { /* ... */ }; // کد قبلی book1
       let book2 = create_book(/* ... */); // کد قبلی book2
       // تغییرات book1
       let book3 = Book { /* ... */ }; // کد قبلی book3 با update syntax

       // دسترسی و نمایش فیلدها با dot notation
       println!("Book 1:");
       println!("  Title: {}", book1.title);
       println!("  Author: {}", book1.author);
       println!("  ISBN: {}", book1.isbn);
       println!("  Published Year: {}", book1.published_year);
       println!("  Is Available: {}", book1.is_available);

       println!("\nBook 2:");
       // توجه: فیلدهای title, author, isbn از book2 به book3 منتقل شده‌اند.
       // تلاش برای دسترسی به book2.title یا book2.author یا book2.isbn در اینجا منجر به خطا می‌شود.
       // فقط فیلدهایی که کپی شده‌اند (مانند published_year و is_available در این مورد خاص)
       // یا مقادیری که کپی نشده‌اند اما در book3 استفاده نشده‌اند (اگر وجود داشته باشند)
       // در book2 قابل دسترسی هستند.
       // در مثال struct update ما، فقط is_available را در book3 مشخص کردیم و بقیه از book2 گرفته شدند.
       // بنابراین title, author, isbn, published_year همگی از book2 به book3 منتقل شدند (به جز published_year که کپی هم می‌شود)
       // و is_available در book2 دست نخورده باقی مانده بود.
       // برای نمایش، می‌توانیم فیلدهایی را که می‌دانیم هنوز در book2 هستند (مانند is_available) یا آن‌هایی که کپی می‌شوند را نمایش دهیم.
       // اما برای سادگی، می‌توانیم قبل از ساخت book3 اطلاعات book2 را چاپ کنیم یا فقط فیلدهایی که مطمئنیم قابل دسترسی هستند را چاپ کنیم.
       // فرض می‌کنیم قبل از ساخت book3 می‌خواستیم اطلاعات book2 را چاپ کنیم:
       //println!("  Title (before update): {}", book2.title); // این خط بعد از ساخت book3 خطا می‌دهد
       //println!("  Author (before update): {}", book2.author); // این خط بعد از ساخت book3 خطا می‌دهد
       //println!("  ISBN (before update): {}", book2.isbn); // این خط بعد از ساخت book3 خطا می‌دهد
       println!("  Published Year: {}", book2.published_year); // OK, u16 is Copy
       println!("  Is Available: {}", book2.is_available); // OK, bool is Copy


       println!("\nBook 3:");
       println!("  Title: {}", book3.title);
       println!("  Author: {}", book3.author);
       println!("  ISBN: {}", book3.isbn);
       println!("  Published Year: {}", book3.published_year);
       println!("  Is Available: {}", book3.is_available);
   }
   ```

**کد کامل نهایی:**

```
struct Book {
    title: String,
    author: String,
    isbn: String,
    published_year: u16,
    is_available: bool,
}

// تابع برای ایجاد نمونه Book با field init shorthand
fn create_book(title: String, author: String, published_year: u16) -> Book {
    Book {
        title, // field init shorthand
        author, // field init shorthand
        published_year, // field init shorthand
        isbn: String::from(""),
        is_available: true,
    }
}

fn main() {
    // 2. ایجاد نمونه قابل تغییر book1
    let mut book1 = Book {
        title: String::from("The Hitchhiker's Guide to the Galaxy"),
        author: String::from("Douglas Adams"),
        isbn: String::from("978-0-345-39180-3"),
        published_year: 1979,
        is_available: true,
    };

    // 4. استفاده از تابع create_book برای ایجاد book2
    let book2 = create_book(
        String::from("The Restaurant at the End of the Universe"),
        String::from("Douglas Adams"),
        1980,
    );

    // 5. تغییر فیلدهای book1 (چون mut است)
    book1.is_available = false;
    book1.isbn = String::from("978-3-16-148410-0");

    // 6. ایجاد book3 با struct update syntax
    // is_available را مشخص می کنیم، بقیه را از book2 می گیریم.
    // توجه: فیلدهای String از book2 به book3 منتقل می شوند.
    let book3 = Book {
        is_available: false, // مشخص کردن فیلد متفاوت
        ..book2 // استفاده از بقیه فیلدها از book2
    };

    // 7. دسترسی و نمایش فیلدها
    println!("Book 1:");
    println!("  Title: {}", book1.title);
    println!("  Author: {}", book1.author);
    println!("  ISBN: {}", book1.isbn);
    println!("  Published Year: {}", book1.published_year);
    println!("  Is Available: {}", book1.is_available);

    println!("\nBook 2:");
    // نمایش فیلدهایی که بعد از struct update هنوز در book2 قابل دسترسی هستند
    println!("  Published Year: {}", book2.published_year); // OK, u16 is Copy
    println!("  Is Available: {}", book2.is_available); // OK, bool is Copy
    // تلاش برای دسترسی به book2.title/author/isbn در اینجا منجر به خطای کامپایلر می شود چون منتقل شده اند.
    // println!("  Title: {}", book2.title); // <-- این خط خطا می دهد

    println!("\nBook 3:");
    println!("  Title: {}", book3.title);
    println!("  Author: {}", book3.author);
    println!("  ISBN: {}", book3.isbn);
    println!("  Published Year: {}", book3.published_year);
    println!("  Is Available: {}", book3.is_available);
}
```

این تکلیف و راه‌حل آن مفاهیم اصلی مربوط به Struct ها در Rust، شامل تعریف، نمونه‌سازی، دسترسی و تغییر فیلدها، استفاده از توابع سازنده، `field init shorthand` و `struct update syntax` را پوشش می‌دهد و همچنین به نکته مهم مالکیت در زمینه Struct ها اشاره می‌کند. منابع همچنین به انواع دیگری از Struct ها مانند Tuple Structs و Unit-Like Structs اشاره دارند که در این تکلیف اصلی گنجانده نشده‌اند اما در منابع به آن‌ها پرداخته شده است. همچنین موضوع Lifetimes برای Struct هایی که شامل رفرنس هستند مطرح شده که در فصل ۱۰ منابع به تفصیل بررسی می‌شوند.


</html>