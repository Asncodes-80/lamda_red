<div align="center" dir="rtl">
    <img src="../../images/logo_lamda_red.png" width="300" height="300" />
    <h1>Lamda RED</h1>
    <p>
        <a href="../en/README.md">English</a> ·
        <a href="../ru/README.md">Русский</a> ·
        <a href="../fa/README.md">فارسی</a> ·
        <a href="../ar/README.md">العربية</a> ·
        <a href="../tr/README.md">Türkçe</a> ·
        <a href="../zh-CN/README.md">简体中文</a> ·
        <a href="../de/README.md">Deutsch</a> ·
        <a href="../es/README.md">Español</a> ·
        <a href="../ko/README.md">한국어</a>
    </p>
    <p>ارسم مخططات هندسة المتطلبات من ملفات Drizzle المصدرية.</p>
</div>

<div dir="rtl">

Lamda RED مُصرّف مكتوب بلغة Rust لصياغة Drizzle النصية التي تصف الأهداف والمخاطر والوكلاء والعلاقات بينها. بدأ المشروع بهدف الاستعاضة عن رسم مخططات المتطلبات بالفأرة في draw.io وMicrosoft Visio. ويسعى إلى جعل Drizzle أداة عملية تُستخدم إلى جانب PlantUML في مشاريع هندسة المتطلبات وهندسة البرمجيات.

يُوزّع المُصرّف باسم `lmdred`. وينشئ الإصدار `1.1.0` صور PNG وملفات mxGraph XML قابلة للتحرير من التخطيط نفسه.

- [الشيفرة المصدرية والتطوير](#source-and-development)
  - [إعداد الشيفرة المصدرية](#set-up-the-source)
  - [التشغيل وتصحيح الأخطاء](#run-and-debug)
  - [الاختبار والتحقق](#test-and-check)
  - [البناء والتحزيم](#build-and-package)
  - [نشر إصدار المُصرّف](#publish-a-compiler-release)
- [التثبيت](#installation)
- [استخدام سطر الأوامر](#cli-usage)
- [صياغة Drizzle](#drizzle-syntax)
- [إضافة Drizzle لـ VS Code](./vscode.md)
- [سجل التغييرات](./CHANGELOG.md)
- [خارطة الطريق](#roadmap)
- [الترخيص](#license)

<a name="source-and-development"></a>

## الشيفرة المصدرية والتطوير

يقرأ المُصرّف ملفًا بترميز UTF-8، ويحلله إلى عقد وروابط، ويقيس التسميات، ويحسب التخطيط والمسارات، ثم يصدّر المخطط.

| المسار | المسؤولية |
| --- | --- |
| [src/main.rs](../../src/main.rs) | وسائط سطر الأوامر ومسارات الملفات والأخطاء ومراحل التصريف |
| [src/lamda_red_grammar.pest](../../src/lamda_red_grammar.pest) | قواعد Drizzle |
| [src/parsing.rs](../../src/parsing.rs) | نموذج المخطط والتعريفات والمراجع وتشخيص أخطاء الصياغة |
| [src/labels.rs](../../src/labels.rs) | اكتشاف الخطوط وتشكيل النص والتفافه واقتطاعه |
| [src/layout.rs](../../src/layout.rs) | هندسة الأشكال وتموضعها على الشبكة وحدود مساحة الرسم |
| [src/routing.rs](../../src/routing.rs) | الروابط والتفرعات والمسارات الالتفافية ومواضع تسميات الروابط |
| [src/export.rs](../../src/export.rs) | إنشاء PNG وmxGraph XML |
| [sample.zz](../../sample.zz) و[sample/](../../sample/) | التحكم بالقطار والمخططات المرجعية الأخرى |
| [tests/test_cli_install.py](../../tests/test_cli_install.py) | اختبارات التكامل لواجهة سطر الأوامر وبرامج التثبيت |
| [scripts/](../../scripts/) | برامج التثبيت وإنشاء أرشيفات الإصدار |
| [.github/workflows/release-cli.yml](../../.github/workflows/release-cli.yml) | البناء والاختبار والنشر لست منصات مستهدفة |
| [drizzle/](../../drizzle/) | إضافة VS Code لتلوين الصياغة |

<a name="set-up-the-source"></a>

### إعداد الشيفرة المصدرية

ثبّت إصدارًا مستقرًا حديثًا من أدوات Rust مع Cargo وأدوات البناء الأصلية لمنصتك. يُستخدم Python 3 لاختبارات تكامل سطر الأوامر وبرامج التثبيت. تعمل أداة تحزيم الإصدار في Windows PowerShell 5.1 أو PowerShell 7. يستخدم إخراج PNG الخطوط المثبتة على النظام. [تثبيت Rust](https://www.rust-lang.org/tools/install).

<div dir="ltr">

```sh
git clone https://github.com/Asncodes-80/lamda_red.git
cd lamda_red
cargo build --locked
```

</div>

يتطلب بناء MSVC على Windows أدوات Visual Studio لبناء C++. على macOS ثبّت أدوات سطر أوامر Xcode. على Linux ثبّت مُصرّف C ورابطًا. تحتاج بيئات Linux المصغّرة أيضًا إلى خط مثل DejaVu Sans.

على Windows مع أدوات GNU، استخدم الأوامر التالية. يمكنك وضع `+stable-x86_64-pc-windows-gnu` مباشرة بعد `cargo` في بقية أوامر البناء والتشغيل والاختبار والتثبيت.

<div dir="ltr">

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal --component clippy,rustfmt
cargo +stable-x86_64-pc-windows-gnu build --locked
```

</div>

أبقِ `Cargo.lock` في المستودع واستخدم `--locked` لاختيار إصدارات الاعتماديات المسجّلة فيه.

<a name="run-and-debug"></a>

### التشغيل وتصحيح الأخطاء

يتضمن بناء التطوير في Cargo معلومات التصحيح. صرّف أحد المخططات المرجعية بالأمر التالي.

<div dir="ltr">

```sh
cargo run --locked -- sample/requirements.zz target/requirements
cargo run --locked -- --help
cargo run --locked -- --version
```

</div>

يكتب الأمر الأول `target/requirements.png` و`target/requirements.xml`. وعند تشغيل `cargo run --locked` وحده، يقرأ `sample.zz` ويكتب `output.png` و`output.xml` في جذر المستودع.

للتصحيح في VS Code، افتح جذر المستودع وثبّت CodeLLDB وأضف الإعداد الآتي إلى `.vscode/launch.json`. يبني الإعداد `lmdred` باستخدام Cargo ويفتح المثال بمسارات نسبية إلى المستودع. [مرجع إعداد CodeLLDB](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#cargo-support).

<div dir="ltr">

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug lmdred",
      "type": "lldb",
      "request": "launch",
      "cargo": {
        "args": ["build", "--locked", "--bin", "lmdred"],
        "filter": { "name": "lmdred", "kind": "bin" }
      },
      "args": ["sample/requirements.zz", "target/debug-diagram"],
      "cwd": "${workspaceFolder}",
      "env": { "RUST_BACKTRACE": "1" },
      "sourceLanguages": ["rust"]
    }
  ]
}
```

</div>

ضع نقطة توقف في الدالة `run` داخل `src/main.rs` واضغط **F5**. ادخل إلى `parsing::parse` لفحص النموذج، وإلى `layout::arrange` لفحص المواضع والمسارات، وإلى `export::png` أو `export::xml` لفحص الإخراج. لاختيار أدوات GNU صراحةً في إعداد التصحيح، غيّر وسائط Cargo إلى `["+stable-x86_64-pc-windows-gnu", "build", "--locked", "--bin", "lmdred"]`.

<a name="test-and-check"></a>

### الاختبار والتحقق

شغّل اختبارات Rust وأداة التنسيق والمدقق من جذر المستودع.

<div dir="ltr">

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

</div>

تغطي اختبارات Rust التحليل والمراجع والعلامات والتفرعات والتوافق مع الإصدارات السابقة وتجاوز العوائق والتفاف النص واقتطاعه.

لاختبار واجهة سطر الأوامر وبرامج التثبيت، ابنِ ملفًا تنفيذيًا للإصدار ومرّر مساره إلى اختبار التكامل.

<div dir="ltr">

```sh
cargo build --locked --release
python tests/test_cli_install.py --binary target/release/lmdred
```

</div>

على Windows استخدم `target/release/lmdred.exe`. إذا حددت هدف Rust صراحةً فاستخدم `target/<target>/release/lmdred` أو `lmdred.exe`. يرسم الاختبار من خارج المستودع، ويفحص المسارات التي تحتوي على مسافات وقيم الإخراج الافتراضية، ويقدّم أرشيف الإصدار عبر HTTP محلي. يثبّت البرنامج في دليل مؤقت ويعيد تثبيته، ويتحقق من رفض قيمة التحقق التالفة مع الإبقاء على الملف التنفيذي المثبّت.

<a name="build-and-package"></a>

### البناء والتحزيم

<div dir="ltr">

```sh
cargo build --locked
cargo build --locked --release
```

</div>

تُكتب ملفات التطوير في `target/debug/` والملفات المحسّنة في `target/release/`. اسم الملف التنفيذي هو `lmdred` على Linux وmacOS و`lmdred.exe` على Windows.

على Windows، ابنِ الهدف ثم حزّمه باستخدام Windows PowerShell 5.1. مع PowerShell 7، استخدم `pwsh` بدلًا من `powershell.exe`.

<div dir="ltr">

```powershell
$compilerTarget = ((rustc -vV | Select-String '^host: ').Line -split ': ', 2)[1]
cargo build --locked --release --target $compilerTarget
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\package-release.ps1 -Target $compilerTarget
```

</div>

يستخدم الأمران هدف سلسلة أدوات Rust النشطة. على Windows x64 يكون الهدف `x86_64-pc-windows-msvc` أو `x86_64-pc-windows-gnu`. يوجد الملف التنفيذي في `target/<target>/release/`. يُكتب الأرشيف وملف `.sha256` في `target/dist/`. يتضمن كل أرشيف الملف التنفيذي والترخيص وREADME و`sample.zz`. تستخدم أهداف Unix أرشيفات `.tar.gz`.

<a name="publish-a-compiler-release"></a>

### نشر إصدار المُصرّف

يبني سير عمل النشر ملفات x64 وARM64 الأصلية ويختبرها على Windows وmacOS وLinux. ويرفع الأرشيفات وقيم تحقق SHA-256 وبرنامجي التثبيت وملف `SHA256SUMS` جامعًا.

بعد تسجيل التغييرات ودفعها، انشر وسمًا مطابقًا للإصدار في `Cargo.toml`.

<div dir="ltr">

```sh
git tag lmdred-v1.1.0
git push origin lmdred-v1.1.0
```

</div>

يشغّل الوسم سير العمل الذي ينشئ الإصدار عند نجاح كل عمليات البناء. كما أن نشر إصدار GitHub يدويًا بوسم المُصرّف نفسه يشغّل سير العمل ويرفق الملفات بذلك الإصدار. يُنشئ زر **Run workflow** مخرجات Actions قابلة للتنزيل للفرع المحدّد.

قبل دفع الوسم للإصدارات التالية، حدّث إصدار المُصرّف في `Cargo.toml` و`Cargo.lock` والإصدار الافتراضي في برنامجي التثبيت وأوامر التثبيت ذات الإصدار المحدد في README. تبدأ وسوم المُصرّف بـ `lmdred-v`. تستخدم إضافة VS Code وسوم `drizzle-` وإصدارها المستقل في `drizzle/package.json`.

<a name="installation"></a>

## التثبيت

تنزّل الأوامر أدناه إصدار المُصرّف `lmdred-v1.1.0` بعد أن يرفع سير عمل النشر ملفاته.

تختار برامج التثبيت x64 أو ARM64، وتُنزّل الأرشيف وقيمة تحقق SHA-256 وتتحقق منه، ثم تثبّت `lmdred` في حساب المستخدم. أعد تشغيل برنامج التثبيت للترقية أو لإصلاح التثبيت.

<a name="linux-and-macos"></a>

### Linux وmacOS

نزّل برنامج التثبيت وشغّله باستخدام curl.

<div dir="ltr">

```sh
curl -fL https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.1.0/install.sh -o install-lmdred.sh
sh install-lmdred.sh
```

</div>

يمكنك تنزيله أيضًا عبر wget.

<div dir="ltr">

```sh
wget https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.1.0/install.sh -O install-lmdred.sh
sh install-lmdred.sh
```

</div>

الدليل الافتراضي هو `$HOME/.local/bin`. يحدّث برنامج التثبيت `~/.profile` وملفات بدء تشغيل الصدفة الحالية Bash أو Zsh أو Fish. افتح طرفية جديدة بعد التثبيت. في Bash أو Zsh أو sh يمكنك تفعيل الأمر فورًا في الطرفية الحالية.

<div dir="ltr">

```sh
export PATH="$HOME/.local/bin:$PATH"
lmdred --version
```

</div>

اختر إصدارًا أو دليل تثبيت بالخيارات التالية.

<div dir="ltr">

```sh
sh install-lmdred.sh --version 1.1.0 --bin-dir "$HOME/.local/bin"
sh install-lmdred.sh --no-modify-path
```

</div>

يحدّد متغيرا البيئة `LMDRED_VERSION` و`LMDRED_INSTALL_DIR` القيم الافتراضية. لخيارات سطر الأوامر الصريحة الأولوية. للصدفات الأخرى، أضف دليل التثبيت إلى إعدادات بدء التشغيل.

<a name="windows"></a>

### Windows

نزّل برنامج التثبيت وشغّله من PowerShell.

<div dir="ltr">

```powershell
$installer = Join-Path $env:TEMP 'install-lmdred.ps1'
Invoke-WebRequest -Uri 'https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.1.0/install.ps1' -OutFile $installer
powershell -NoProfile -ExecutionPolicy Bypass -File $installer
```

</div>

افتح طرفية جديدة وشغّل `lmdred --version`. يضع برنامج التثبيت الملف التنفيذي في `%LOCALAPPDATA%\Programs\lmdred\bin` ويضع دليله في مقدمة PATH الخاص بالمستخدم، فيسبق النسخة المثبتة عبر Cargo. إذا سبقت نسخة مثبّتة على مستوى النظام، يعرض البرنامج مسارها. استخدم `Get-Command lmdred -All` لمعرفة النسخة التي تختارها الطرفية.

اختر موقعًا آخر أو أدر PATH بنفسك بهذه الخيارات.

<div dir="ltr">

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -Version 1.1.0 -InstallDirectory "$env:LOCALAPPDATA\Programs\lmdred\bin"
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -NoModifyPath
```

</div>

يحدّد `LMDRED_VERSION` و`LMDRED_INSTALL_DIR` القيم الافتراضية على Windows أيضًا.

<a name="download-an-archive-directly"></a>

### تنزيل الأرشيف مباشرةً

فكّ أرشيف نظامك وضع الملف التنفيذي في أحد أدلة PATH.

| النظام | المعالج | أرشيف الإصدار |
| --- | --- | --- |
| Windows | x64 | `lmdred-v1.1.0-x86_64-pc-windows-msvc.zip` |
| Windows | ARM64 | `lmdred-v1.1.0-aarch64-pc-windows-msvc.zip` |
| macOS | Intel | `lmdred-v1.1.0-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon | `lmdred-v1.1.0-aarch64-apple-darwin.tar.gz` |
| Linux | x64 | `lmdred-v1.1.0-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 | `lmdred-v1.1.0-aarch64-unknown-linux-musl.tar.gz` |

تستخدم ملفات Linux مكتبة musl لدعم توزيعات glibc وmusl. تُربط مكتبة تشغيل C ربطًا ساكنًا في إصدارات Windows. يعتمد الرسم على خطوط النظام. على خادم Linux مصغّر أو في حاوية، ثبّت خطًا بلا زوائد مثل DejaVu Sans.

<a name="install-from-source"></a>

### التثبيت من المصدر

بعد تثبيت Rust وأدوات البناء، شغّل الأمر التالي من جذر المستودع.

<div dir="ltr">

```sh
cargo install --path . --locked
```

</div>

يثبّت Cargo الأمر `lmdred` في `~/.cargo/bin`، أو في `%USERPROFILE%\.cargo\bin` على Windows. أضف الدليل إلى PATH عند الحاجة. يمكنك أيضًا التثبيت من الشيفرة على GitHub مباشرةً.

<div dir="ltr">

```sh
cargo install --git https://github.com/Asncodes-80/lamda_red --locked --bin lmdred
```

</div>

استخدم الأمر التالي لأدوات GNU على Windows.

<div dir="ltr">

```powershell
cargo +stable-x86_64-pc-windows-gnu install --path . --locked
```

</div>

<a name="cli-usage"></a>

## استخدام سطر الأوامر

يمكنك تشغيل الأمر المثبّت من أي دليل.

<div dir="ltr">

```text
lmdred [input.zz] [output-stem]
lmdred --help
lmdred --version
```

</div>

| الوسيط أو الخيار | المعنى | القيمة الافتراضية |
| --- | --- | --- |
| `input.zz` | مسار ملف Drizzle بترميز UTF-8 | `sample.zz` في الدليل الحالي |
| `output-stem` | مسار الإخراج قبل إلحاق الامتدادين `.png` و`.xml` | `output` في الدليل الحالي |
| `-h`, `--help` | عرض مساعدة سطر الأوامر |  |
| `-V`, `--version` | عرض إصدار المُصرّف، مثل `lmdred 1.1.0` |  |
| `--` | معاملة الوسائط المتبقية كمسارات ملفات |  |

<a name="output-paths"></a>

### مسارات الإخراج

تُفسّر المسارات النسبية انطلاقًا من دليل عمل الطرفية الحالي. أعطِ مسارًا مطلقًا لاختيار موقع آخر. يجب أن يكون دليل الإخراج موجودًا. يلحق المُصرّف الامتدادات بالاسم الأساسي المعطى.

| الأمر | الملفات المكتوبة نسبةً إلى الدليل الحالي |
| --- | --- |
| `lmdred sample.zz my-new-red` | `my-new-red.png` و`my-new-red.xml` |
| `lmdred sample.zz` | `output.png` و`output.xml` |
| `lmdred` | يقرأ `sample.zz` ويكتب `output.png` و`output.xml` |
| `lmdred diagrams/train.zz` | `output.png` و`output.xml` |
| `lmdred diagrams/train.zz reports/train` | `reports/train.png` و`reports/train.xml` |
| `lmdred sample.zz train.v1` | `train.v1.png` و`train.v1.xml` |

ضع المسارات التي تحتوي على مسافات بين علامتي اقتباس. استخدم `--` إذا بدأ المسار بـ `-`.

<div dir="ltr">

```sh
lmdred "my diagrams/train.zz" "my diagrams/train"
lmdred -- "-file.zz" result
```

</div>

<a name="output-formats-and-errors"></a>

### صيغ الإخراج والأخطاء

تحتوي صورة PNG على المخطط بمساحة رسم تتّسع للأشكال والروابط والتسميات. ويضم mxGraph XML الأشكال والروابط ونقاط المسار القابلة للتحرير في diagrams.net/draw.io.

عند نجاح التصريف يُطبع مسارا الإخراج ويكون رمز الخروج `0`. عند الفشل تُكتب رسالة في تدفّق الخطأ القياسي ويكون الرمز `1`. تتضمن أخطاء الصياغة والمراجع اسم الملف والسطر والعمود.

تُستبدل ملفات الإخراج الموجودة عند نجاح الكتابة. يتحقق المُصرّف من أن مسار الإخراج لا يستبدل الملف المصدري. يحدث التحليل والرسم قبل كتابة أي إخراج. قد يترك خطأ نظام الملفات أحد الملفين مكتوبًا. راجع الرسالة وأعد التشغيل بعد تصحيح المسار أو الأذونات.

<a name="drizzle-syntax"></a>

## صياغة Drizzle

استخدم PNG في المستندات والعروض. احتفظ بملف Drizzle مع المتطلبات لتمكين المراجعين من تتبّع الأهداف والمخاطر والوكلاء والعلاقات وتعديلها. ينتج كل تصريف أيضًا ملف mxGraph XML قابلًا للتحرير.

<a name="a-first-diagram"></a>

### المخطط الأول

احفظ المثال باسم `first.zz` وشغّل `lmdred first.zz first`.

<div dir="ltr">

```drizzle
&& The tracking agent reports train speed and location.
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
```

</div>

يتضمن الناتج وكيلًا سداسيًا وكائنًا مستطيلًا وسهمًا متصلًا يشير إلى القطار. تتكون التسمية فوق السهم من سطرين.

<a name="declare-nodes"></a>

### تعريف العقد

اكتب `ID = shape` لتمكين الإشارة إلى العقدة في رابط. تختلف المعرّفات باختلاف حالة الأحرف وتبدأ بحرف ASCII وتقبل الحروف والأرقام والشرطة السفلية المفردة بين حرفين أو رقمين أو بين حرف ورقم. `Train_1` مثال على معرّف. النص داخل الشكل هو التسمية المعروضة. يمكن وضع الروابط قبل التعريفات أو بعدها. استخدم `&&` في أول السطر للتعليق. يُسمح بالمسافات البادئة والأسطر الفارغة.

يمكن كتابة الأشكال بمفردها، مثل `/Safe journey/`. عيّن معرّفًا للشكل إذا أردت استخدامه في رابط.

| الشكل | التعريف | النتيجة |
| --- | --- | --- |
| هدف | `G = /Safe journey/` | متوازي أضلاع مائل إلى اليمين |
| خطر | `R = \Signal lost\` | متوازي أضلاع معكوس |
| وكيل | `A = <Train controller>` | سداسي الأضلاع |
| عبارة وصفية | `D = ~Doors are locked in motion~` | شبه منحرف بقاعدة أعرض |
| كائن | `O = [Train]` | مستطيل |
| إجراء | `Send = (Send command)` | شكل بيضاوي |
| ملاحظة | `N = {Controller responsibility}` | مستطيل أخضر فاتح بإطار متقطع |
| تعليق سحابي | `C = {{Reason for the obstacle}}` | إطار على هيئة سحابة |
| معين | `Check = ^Speed confirmed?^` | شكل معيّن |

ضع `!` قبل الشكل لتغليظ إطاره. يُستخدم هذا الترميز في الأمثلة للأوراق مثل المتطلبات والافتراضات. تبقى سماكة نص التسمية عادية.

<div dir="ltr">

```drizzle
/Goal/
!/Leaf requirement/
~Descriptive~
<Agent 1>
```

</div>

<a name="labels-and-line-breaks"></a>

#### التسميات وفواصل الأسطر

تدعم التسميات UTF-8 بما فيه النص من اليمين إلى اليسار. يقيس العارض النص ويوسّطه ويضبط الشكل لاحتوائه. يُلف النص الطويل بين الكلمات متى أمكن، وقد تُقسّم الكلمة الطويلة نفسها. تُعرض أربعة أسطر على الأكثر لتسمية الشكل وثلاثة لتسمية الرابط. ينتهي النص الذي يتجاوز الحد بـ `...` وتبقى التسمية الكاملة في المصدر.

| الصياغة | النتيجة |
| --- | --- |
| `/Accurate estimate\nof speed and position/` | تسمية هدف بفاصل سطر |
| `[C:\\Logs]` | مستطيل بتسمية `C:\Logs` |
| `<Speed \> zero>` | وكيل بتسمية `Speed > zero` |
| `/Input \/ output/` | هدف بتسمية `Input / output` |
| `~Value \~ threshold~` | شبه منحرف بتسمية `Value ~ threshold` |

اكتب `\n` لفرض فاصل سطر و`\\` لعرض شرطة مائلة عكسية. ضع شرطة مائلة عكسية قبل أي محدّد تريد عرضه داخل التسمية.

<a name="connect-nodes"></a>

### ربط العقد

استخدم معرّفات العقد على طرفي العامل. يختار المحرفان الأوسطان نمط الخط، ويضيف `<` و`>` رأسَي سهم إلى الطرفين الأيسر والأيمن على الترتيب. أضف مسافات بين المعرّفات والعامل عند الحاجة إلى تحسين القراءة.

| الاتجاه | متصل | متقطع |
| --- | --- | --- |
| بلا رؤوس أسهم | `A __ B` | `A -- B` |
| نحو B | `A__>B` | `A-->B` |
| نحو A | `A<__B` | `A<--B` |
| نحو الطرفين | `A<__>B` | `A<-->B` |

اكتب نقطتين رأسيتين بعد الرابط وخياراته لإضافة تسمية. يمكن لرابطين بين العقدتين نفسيهما أن يحملا تسميتين مختلفتين ويسلكا مسارين منفصلين. الرابط المكرر يُرسم مرة واحدة. ربط العقدة بنفسها ينشئ حلقة.

<div dir="ltr">

```drizzle
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train --> Train : Refresh state
```

</div>

<a name="add-notation-to-a-connection"></a>

#### إضافة علامة للرابط

| الخيار | المثال | النتيجة |
| --- | --- | --- |
| `[o]` | `A __ B [o]` | دائرة فارغة على الخط |
| `[*]` | `A __> B [*]` | دائرة ممتلئة على الخط |
| `[zigzag]` | `A __ B [zigzag]` | خط متعرج للتعارض |
| `[bar]` | `Risk --> Goal [bar]` | شرطة عرضية للعائق |
| `[ortho]` | `A __> B [ortho]` | مسار مركب من مقاطع أفقية وعمودية |

استخدم دائرة أو تعرجًا أو شرطة عرضية واحدة في كل رابط. يمكن الجمع بينها وبين `[ortho]` وأيّ من نمطي الخط. تظهر الشرطة العرضية قرب رأس السهم في الرابط ذي السهم الواحد، وقرب منتصف المسار في الرابط بلا سهم أو ذي السهمين.

<a name="join-several-sources"></a>

#### دمج عدة مصادر

تشترك الروابط المتجهة إلى الهدف نفسه في المقطع الأخير إذا تطابق نمط الخط والعلامة والتسمية وخيارات المسار. تبقى المسارات الالتفافية المحددة صراحة مستقلة. لتعريف فرع مشترك مباشرةً، افصل معرّفات المصادر بفواصل. تظهر الدائرة الفارغة أو الممتلئة عند نقطة الالتقاء، ويتجه السهم الأيمن إلى الهدف، والأيسر إلى كل مصدر في الفرع الصريح.

<div dir="ltr">

```drizzle
layout up
Goal @1,0 = /Safe departure/
Software @0,1 = !/Verify the lock signal/
Assumption @2,1 = !/The door sensor works/

Software, Assumption __> Goal [o]
```

</div>

تلتقي العقدتان الطرفيتان على خط مشترك بدائرة فارغة وسهم واحد نحو الهدف أعلاهما. تعطي كتابة `Software __> Goal [o]` و`Assumption __> Goal [o]` في سطرين منفصلين النتيجة نفسها.

<a name="set-the-layout-and-routing"></a>

### التخطيط وتوجيه المسارات

| التحكم | المثال | النتيجة |
| --- | --- | --- |
| الاتجاه | `layout right` | تخطيط تلقائي من اليسار إلى اليمين، وهو الافتراضي |
| الاتجاه | `layout left` | تخطيط تلقائي من اليمين إلى اليسار |
| الاتجاه | `layout up` | تخطيط تلقائي من الأسفل إلى الأعلى بمسارات قائمة الزوايا |
| الاتجاه | `layout down` | تخطيط تلقائي من الأعلى إلى الأسفل بمسارات قائمة الزوايا |
| موضع على الشبكة | `Goal @2,0 = /Goal/` | يضع الشكل في العمود 2 والصف 0 |
| مسار خارجي | `A --> B [bar] via bottom` | يمرّر الرابط من أسفل المخطط |

اكتب توجيه تخطيط واحدًا على الأكثر لكل مخطط. تبدأ إحداثيات الشبكة من الصفر، وتزداد الأعمدة يمينًا والصفوف نزولًا. تتسع كل خلية محددة صراحةً لعقدة واحدة. يضبط Lamda RED الصفوف والأعمدة لتلائم الأشكال ومساحة PNG لتلائم المخطط. يمكن الجمع بين التموضع التلقائي والصريح.

يضع الرسم أدناه الكائنات والوكيل في الصف صفر. توضح الخطوط المنقطة ونقاط التقاطع الإحداثيات. التقاطع المميز `@0,2` في العمود صفر والصف اثنين.

![مخطط الكائنات والوكيل فوق شبكة منقطة مرقمة من الصفر](../../docs/images/grid-placement.png)

<div dir="ltr">

```drizzle
Input @0,0 = [Object (obj1)]
Monitor @1,0 = <Agent (Ag)>
Output @2,0 = [Object (obj2)]

Input __> Monitor : Obj1.attribute1\nValue monitoring
Monitor __> Output : Obj2.attribute2\nState variable\nControl
```

</div>

لوضع ملاحظة عند التقاطع المميز أضف `Explanation @0,2 = {Column zero, row two}`. تحدد الإحداثيات الخلية، أما موضعها بالبكسل فيعتمد على أحجام الأشكال.

اكتب `via right` أو `via left` أو `via top` أو `via bottom` بعد خيارات الرابط وقبل تسميته لتمرير علاقة تعارض أو عائق حول العلاقات الأخرى.

<div dir="ltr">

```drizzle
layout up
Login @0,0 = /User can sign in/
Blocked @0,1 = \User cannot sign in\
Recover @0,2 = /Recover using a phone number/

Recover __> Blocked
Blocked __> Login
Recover __> Login [zigzag] via right : A non-user might gain access
```

</div>

يتجه الرابطان الأولان إلى أعلى الشجرة. يمرّ التعارض من اليمين ويعود نحو الهدف.

<a name="reference-diagrams"></a>

### مخططات مرجعية

يعرض كل مثال ناتج تصريف ملف Drizzle المرتبط به. شغّل `lmdred sample/de-morgan.zz de-morgan` لمخطط دي مورغان أو استبدل مسار المثال والاسم الأساسي للإخراج.

<a name="goal-notation-and-obstacles"></a>

#### ترميز الأهداف والعوائق

يميز الإطار السميك العقدة الطرفية. يسجّل شبه المنحرف عبارة وصفية، ويشير شكل الخطر المعكوس إلى الهدف عبر شرطة عرضية تدل على عائق. تحمل السحابة الملاحظة التوضيحية. [مصدر Drizzle](../../sample/notation.zz).

![ترميز أهداف ومخاطر يتضمن سحابة وشبه منحرف وسداسيًا وشرطة عرضية](../../docs/images/notation.png)

<a name="requirement-refinement"></a>

#### تفصيل المتطلبات

تتيح الروابط المستقلة وضع علامات ورؤوس أسهم خاصة بالمتطلب المفصّل والمتطلبات الطرفية. [مصدر Drizzle](../../sample/requirements.zz).

![شجرة تفصيل من الأسفل إلى الأعلى بثلاث عقد طرفية سميكة الإطار](../../docs/images/requirements.png)

<a name="rail-goal-conflicts"></a>

#### تعارض أهداف السكك الحديدية

تجمع الفروع عدة تفصيلات عند نقطة واحدة. وتربط الخطوط المتعرجة الأهداف المتعارضة عبر الشجرة. [مصدر Drizzle](../../sample/rail-conflicts.zz).

![مخطط أهداف السكك الحديدية بنقاط التقاء دائرية وتعارضين متعرجين](../../docs/images/rail-conflicts.png)

<a name="authentication-risk"></a>

#### خطر المصادقة

تشير الأشكال المعكوسة إلى مشكلات تمنع تسجيل الدخول. ويبين التعرج الخارجي تعارضًا محتملاً ناتجًا عن خيار استعادة الحساب. [مصدر Drizzle](../../sample/auth-risk.zz).

![شجرة خطر المصادقة بمسار متعرج يلتف حول الجانب الأيمن](../../docs/images/auth-risk.png)

<a name="de-morgan-goal-and-risk-diagram"></a>

#### مخطط دي مورغان للأهداف والمخاطر

يعرّف المصدر صراحةً الهدف ونفيه وفرع التفصيل المشترك والعوائق المتقطعة الثلاثة. [مصدر Drizzle](../../sample/de-morgan.zz).

![هدف ونفيه بفرع مشترك وروابط متقطعة بعلامات عائق](../../docs/images/de-morgan.png)

<a name="agent-monitoring-and-control"></a>

#### المراقبة والتحكم بواسطة الوكيل

يستقبل الوكيل السداسي خاصية من الكائن الأول ويتحكم بخاصية في الثاني. [مصدر Drizzle](../../sample/object-agent.zz).

![كائن الإدخال يتصل بالوكيل، والوكيل يتصل بكائن الإخراج](../../docs/images/object-agent.png)

<a name="train-control"></a>

#### التحكم بالقطار

يجمع المخطط خطوطًا متصلة ومتقطعة وأسهمًا منفصلة للقياسات والأوامر وإجراءً بيضاويًا وملاحظة عن المسؤولية. [مصدر Drizzle](../../sample.zz).

![متطلبات التحكم بالقطار مع وكلاء وكائنات وإجراء وملاحظة عن المسؤولية](../../output.png)

<a name="editor-and-diagnostics"></a>

### المحرر وتشخيص الأخطاء

تلوّن إضافة VS Code في `drizzle/` صياغة ملفات `.zz` و`.drizzle` و`.rd`. لتجربة القواعد الحالية، افتح المجلد في VS Code واضغط **F5** لتشغيل Extension Development Host. افتح مصدر المخطط في نافذة المضيف. [مستندات إضافات VS Code](https://code.visualstudio.com/api/get-started/your-first-extension).

| الرسالة أو العَرَض | الحل |
| --- | --- |
| `unknown node 'X'` | عرّف `X` بحالة الأحرف نفسها أو صحّح المرجع. |
| `duplicate ID` | أعطِ كل عقدة معرّفة معرّفًا فريدًا. |
| `cell @… is already occupied` | انقل إحدى العقدتين إلى خلية أخرى. |
| `invalid Drizzle syntax` | استخدم الملف والسطر والعمود الواردة في الرسالة للعثور على المحدّد أو العامل أو الخيار في الموضع الخاطئ. |
| `diagram is too large for PNG output` | قرّب مواضع الشبكة المتباعدة أو قسّم النموذج إلى عدة مخططات. |
| `no system fonts found` | ثبّت خطًا بلا زوائد وأعد تشغيل المُصرّف. |
| `link.exe not found` | ثبّت أدوات بناء MSVC أو شغّل أمر GNU أعلاه. |

<a name="roadmap"></a>

## خارطة الطريق

- إنشاء مخطط آلة حالات لقواعد Pest في Lamda RED.
- تطوير خدمة ويب بواجهة React تطلب صور PNG من خادم Rust.

<a name="license"></a>

## الترخيص

يُرخّص هذا المشروع بموجب [GNU GPLv3](../../LICENSE).

</div>
