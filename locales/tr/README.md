<div align="center">
    <img src="../../images/logo_lamda_red.png" width="128" height="128" />
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
    <p>Düz metinden gereksinim mühendisliği diyagramları</p>
</div>

Lamda RED, düz metin `.zz` dosyalarını gereksinim diyagramlarına dönüştürür. Sistemin hangi hedeflere ulaşması gerektiğini, karşılaşabileceği riskleri ve aktörlerle sistem nesnelerinin nasıl etkileştiğini gösterin. Ekibiniz değişiklikleri Git diff ile inceleyebilsin ve görselleri yeniden oluşturabilsin diye kaynak dosyayı gereksinimlerle birlikte saklayın.

Diyagramı **Drizzle** ile yazın ve **`lmdred`** ile derleyin. Derleyici belgeler için PNG ve diagrams.net (draw.io) için düzenlenebilir mxGraph XML üretir. `--export-set` seçeneği küçük, belgeye uygun ve yüksek çözünürlüklü üç PNG üretir. Masaüstü VS Code uzantısı diyagramı düzenleyicinin yanında gösterir ve kaydederken veya **Export diagram** düğmesiyle dışa aktarır.

## Drizzle'dan diyagrama

Bu örnekte bir aktör, bir nesneden aldığı değeri izliyor ve başka bir nesneyi denetliyor. Üretilen diyagram aşağıda.

![Girdi nesnesi aktöre, aktör de çıktı nesnesine bağlanır](../../docs/images/object-agent.png)

Örneği `object-agent.zz` olarak kaydedin.

```drizzle
Input = [Object (obj1)]
Monitor = <Agent (Ag)>
Output = [Object (obj2)]

Input __> Monitor : Obj1.attribute1\nValue monitoring
Monitor __> Output : Obj2.attribute2\nState variable\nControl
```

Derleyiciyi kurduktan sonra komutu dosyanın dizininde çalıştırın. `object-agent.png` ve `object-agent.xml` oluşturulur.

```sh
lmdred object-agent.zz object-agent
```

[Kurulum](#installation) · [VS Code önizlemesi](../../README.md#live-preview-in-vs-code) · [Drizzle sözdizimi kılavuzu](#drizzle-syntax)

- [CLI kullanımı](#cli-usage)
- [Drizzle sözdizimi](#drizzle-syntax)
- [VS Code için Drizzle uzantısı](./vscode.md)
- [Kurulum](#installation)
- [Kaynak kod ve geliştirme](#source-and-development)
  - [Kaynak kodu hazırlama](#set-up-the-source)
  - [Çalıştırma ve hata ayıklama](#run-and-debug)
  - [Test ve denetim](#test-and-check)
  - [Derleme ve paketleme](#build-and-package)
  - [Derleyici sürümü yayımlama](#publish-a-compiler-release)
- [Değişiklik günlüğü](./CHANGELOG.md)
- [Yol haritası](#roadmap)
- [Lisans](#license)

<a name="cli-usage"></a>

## CLI kullanımı

Kurulu komutu herhangi bir dizinden çalıştırabilirsiniz.

```text
lmdred [input.zz] [output-stem]
lmdred --help
lmdred --version
```

| Argüman veya seçenek | Anlamı | Varsayılan |
| --- | --- | --- |
| `input.zz` | UTF-8 Drizzle kaynak dosyasının yolu | Geçerli dizindeki `sample.zz` |
| `output-stem` | `.png` ve `.xml` uzantıları eklenmeden önceki çıktı yolu | Geçerli dizindeki `output` |
| `-h`, `--help` | Komut satırı yardımını yazdırır |  |
| `-V`, `--version` | Derleyici sürümünü yazdırır, örneğin `lmdred 1.2.0` |  |
| `--` | Kalan argümanları dosya yolu olarak işler |  |

<a name="output-paths"></a>

### Çıktı yolları

Göreli yollar terminalin çalışma dizinine göre çözülür. Başka bir konum için mutlak yol verin. Çıktı dizini önceden var olmalıdır. Derleyici verilen temel ada uzantıları ekler.

| Komut | Geçerli dizine göre yazılan dosyalar |
| --- | --- |
| `lmdred sample.zz my-new-red` | `my-new-red.png` ve `my-new-red.xml` |
| `lmdred sample.zz` | `output.png` ve `output.xml` |
| `lmdred` | `sample.zz` dosyasını okur, `output.png` ve `output.xml` dosyalarını yazar |
| `lmdred diagrams/train.zz` | `output.png` ve `output.xml` |
| `lmdred diagrams/train.zz reports/train` | `reports/train.png` ve `reports/train.xml` |
| `lmdred sample.zz train.v1` | `train.v1.png` ve `train.v1.xml` |

Boşluk içeren yolları tırnak içine alın. Yol `-` ile başlıyorsa `--` kullanın.

```sh
lmdred "my diagrams/train.zz" "my diagrams/train"
lmdred -- "-file.zz" result
```

<a name="output-formats-and-errors"></a>

### Çıktı biçimleri ve hatalar

PNG tuvali şekilleri, bağlantıları ve etiketleri içine alır. mxGraph XML, diagrams.net/draw.io için düzenlenebilir şekiller, bağlantılar ve ara noktalar içerir.

Derleme başarılıysa iki çıktı yolu yazdırılır ve çıkış durumu `0` olur. Hatalar standart hata akışına yazılır ve durum `1` olur. Sözdizimi ve başvuru hataları dosya, satır ve sütun bilgisi içerir.

Yazma başarılı olduğunda var olan çıktı dosyaları üzerine yazılır. Derleyici bir çıktı yolunun kaynak dosyanın yerini almamasını sağlar. İki çıktı yazılmadan önce ayrıştırma ve çizim biter. Kayıt sırasında dosya sistemi hatası olursa çıktılardan biri yazılmış olabilir. Hata iletisini inceleyip yol veya izinleri düzelttikten sonra yeniden çalıştırın.

<a name="drizzle-syntax"></a>

## Drizzle sözdizimi

PNG'yi belge ve sunumlarda kullanın. İnceleyenlerin hedefleri, riskleri, aktörleri ve ilişkileri takip edip düzeltebilmesi için Drizzle dosyasını gereksinimlerle birlikte saklayın. Her derleme düzenlenebilir mxGraph XML de üretir.

<a name="a-first-diagram"></a>

### İlk diyagram

Örneği `first.zz` olarak kaydedip `lmdred first.zz first` komutunu çalıştırın.

```drizzle
&& The tracking agent reports train speed and location.
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
```

Çıktıda altıgen bir aktör, dikdörtgen bir nesne ve treni gösteren düz çizgili bir ok vardır. Okun üstündeki etiket iki satırlıdır.

<a name="declare-nodes"></a>

### Düğüm tanımlama

Bağlantılarda düğüme başvurmak için `ID = shape` yazın. Kimlikler büyük/küçük harfe duyarlıdır, ASCII harfle başlar ve harf, rakam ve alfanümerik karakterler arasında tek alt çizgi kabul eder. `Train_1` bir kimliktir. Şeklin içindeki metin etikettir. Bağlantılar tanımdan önce veya sonra gelebilir. Yorum için satır başına `&&` yazın. Boş satır ve girinti kullanılabilir.

`/Safe journey/` gibi tek başına şekiller de yazabilirsiniz. Bağlantıda kullanmak istediğiniz şekle bir kimlik verin.

| Şekil | Tanım | Sonuç |
| --- | --- | --- |
| Hedef | `G = /Safe journey/` | Sağa eğimli paralelkenar |
| Risk | `R = \Signal lost\` | Ters eğimli paralelkenar |
| Aktör | `A = <Train controller>` | Altıgen |
| Betimleyici ifade | `D = ~Doors are locked in motion~` | Tabanı daha geniş yamuk |
| Nesne | `O = [Train]` | Dikdörtgen |
| Eylem | `Send = (Send command)` | Elips |
| Not | `N = {Controller responsibility}` | Kesik kenarlı açık yeşil dikdörtgen |
| Bulut açıklaması | `C = {{Reason for the obstacle}}` | Bulut biçimli çerçeve |
| Eşkenar dörtgen | `Check = ^Speed confirmed?^` | Eşkenar dörtgen |

Çerçeveyi kalınlaştırmak için şeklin başına `!` ekleyin. Referans gösteriminde gereksinim ve varsayım gibi yapraklar bu şekilde işaretlenir. Etiketin yazı kalınlığı normal kalır.

```drizzle
/Goal/
!/Leaf requirement/
~Descriptive~
<Agent 1>
```

<a name="labels-and-line-breaks"></a>

#### Etiketler ve satır sonları

Etiketler sağdan sola metin dahil UTF-8 kabul eder. Oluşturucu metni ölçer, ortalar ve şekli metne göre boyutlandırır. Uzun etiketler mümkün olduğunda sözcük sınırlarında kayar. Uzun sözcükler kendi içinde bölünebilir. Şekil etiketi en fazla dört, bağlantı etiketi en fazla üç satır gösterir. Sınırı aşan metin `...` ile biter. Tam etiket kaynakta kalır.

| Sözdizimi | Sonuç |
| --- | --- |
| `/Accurate estimate\nof speed and position/` | Satır sonu bulunan hedef etiketi |
| `[C:\\Logs]` | `C:\Logs` etiketli dikdörtgen |
| `<Speed \> zero>` | `Speed > zero` etiketli aktör |
| `/Input \/ output/` | `Input / output` etiketli hedef |
| `~Value \~ threshold~` | `Value ~ threshold` etiketli yamuk |

Zorunlu satır sonu için `\n`, ters eğik çizgiyi göstermek için `\\` yazın. Etikette göstermek istediğiniz ayırıcının önüne ters eğik çizgi koyun.

<a name="connect-nodes"></a>

### Düğümleri bağlama

İşlecin iki yanında adlandırılmış düğümler kullanın. Ortadaki iki karakter çizgi türünü seçer. Dıştaki `<` ve `>` sırasıyla sol ve sağ uçlara ok ucu ekler. Okunabilirlik için kimliklerle işleç arasına boşluk koyabilirsiniz.

| Yön | Düz | Kesikli |
| --- | --- | --- |
| Ok ucu yok | `A __ B` | `A -- B` |
| B'ye doğru | `A__>B` | `A-->B` |
| A'ya doğru | `A<__B` | `A<--B` |
| İki uca doğru | `A<__>B` | `A<-->B` |

Etiket yazmak için bağlantı ve seçeneklerinden sonra iki nokta koyun. Aynı düğümler arasındaki iki bağlantı farklı etiketler ve rotalar kullanabilir. Yinelenen bağlantı bir kez çizilir. Bir düğümü kendisine bağlamak döngü oluşturur.

```drizzle
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train --> Train : Refresh state
```

<a name="add-notation-to-a-connection"></a>

#### Bağlantıya işaret ekleme

| Seçenek | Örnek | Sonuç |
| --- | --- | --- |
| `[o]` | `A __ B [o]` | Çizgi üzerinde içi boş daire |
| `[*]` | `A __> B [*]` | Çizgi üzerinde dolu daire |
| `[zigzag]` | `A __ B [zigzag]` | Çatışmayı belirten zikzak |
| `[bar]` | `Risk --> Goal [bar]` | Engeli belirten enine çizgi |
| `[ortho]` | `A __> B [ortho]` | Yatay ve düşey parçalardan oluşan rota |

Bağlantı başına bir daire, zikzak veya enine çizgi kullanın. İşareti `[ortho]` ve her iki çizgi türüyle birleştirebilirsiniz. Enine çizgi tek oklu bağlantıda ok ucuna, oksuz veya çift oklu bağlantıda rotanın ortasına yakın yerleştirilir.

<a name="join-several-sources"></a>

#### Birden fazla kaynağı birleştirme

Aynı hedefe yönelen bağlantılar çizgi türü, işaret, etiket ve rota seçenekleri eşleştiğinde son parçayı paylaşır. Açıkça tanımlanan dış rotalar ayrı kalır. Paylaşılan bir bağlantıyı doğrudan tanımlamak için kaynak kimliklerini virgülle ayırın. Boş veya dolu daire birleşimdedir, sağ ok ucu hedefe girer ve sol ok ucu açık dalın her kaynağına yönelir.

```drizzle
layout up
Goal @1,0 = /Safe departure/
Software @0,1 = !/Verify the lock signal/
Assumption @2,1 = !/The door sensor works/

Software, Assumption __> Goal [o]
```

İki yaprak düğüm, boş daireli ortak çizgiye bağlanır ve tek ok yukarıdaki hedefe girer. `Software __> Goal [o]` ve `Assumption __> Goal [o]` ifadelerini ayrı satırlarda yazmak aynı sonucu verir.

<a name="set-the-layout-and-routing"></a>

### Yerleşim ve yönlendirme

| Denetim | Örnek | Sonuç |
| --- | --- | --- |
| Yön | `layout right` | Soldan sağa otomatik akış, varsayılan |
| Yön | `layout left` | Sağdan sola otomatik akış |
| Yön | `layout up` | Dik açılı rotalarla aşağıdan yukarı otomatik akış |
| Yön | `layout down` | Dik açılı rotalarla yukarıdan aşağı otomatik akış |
| Izgara yerleşimi | `Goal @2,0 = /Goal/` | Şekli 2. sütuna, 0. satıra yerleştirir |
| Dış rota | `A --> B [bar] via bottom` | Bağlantıyı diyagramın altından dolaştırır |

Diyagram başına en fazla bir yerleşim direktifi yazın. Izgara koordinatları sıfırdan başlar, sütunlar sağa ve satırlar aşağı doğru artar. Açıkça seçilen hücrede bir düğüm yer alır. Lamda RED satır ve sütunları şekillere, PNG tuvalini de diyagrama sığacak biçimde boyutlandırır. Otomatik ve açık yerleşimi birlikte kullanabilirsiniz.

Aşağıdaki görsel nesne ve aktör örneğini sıfırıncı satıra yerleştirir. Noktalı çizgiler ve kesişim noktaları ızgara koordinatlarını gösterir. Vurgulanan `@0,2` noktası sıfırıncı sütunda ve ikinci satırdadır.

![Sıfırdan numaralanan noktalı ızgarada nesne ve aktör diyagramı](../../docs/images/grid-placement.png)

```drizzle
Input @0,0 = [Object (obj1)]
Monitor @1,0 = <Agent (Ag)>
Output @2,0 = [Object (obj2)]

Input __> Monitor : Obj1.attribute1\nValue monitoring
Monitor __> Output : Obj2.attribute2\nState variable\nControl
```

Vurgulanan kesişime not koymak için `Explanation @0,2 = {Column zero, row two}` ekleyin. Koordinatlar hücreyi seçer ve piksel konumu şekillerin boyutlarına bağlıdır.

Bağlantı seçeneklerinden sonra ve etiketten önce `via right`, `via left`, `via top` veya `via bottom` yazın. Böylece çatışma veya engel diğer ilişkilerin çevresinden dolaşabilir.

```drizzle
layout up
Login @0,0 = /User can sign in/
Blocked @0,1 = \User cannot sign in\
Recover @0,2 = /Recover using a phone number/

Recover __> Blocked
Blocked __> Login
Recover __> Login [zigzag] via right : A non-user might gain access
```

İlk iki bağlantı ağaçta yukarı yönelir. Çatışma sağdan dolaşarak hedefi gösterir.

<a name="reference-diagrams"></a>

### Referans diyagramları

Her örnek, bağlantıdaki Drizzle dosyasının derleme çıktısını gösterir. De Morgan diyagramı için `lmdred sample/de-morgan.zz de-morgan` komutunu çalıştırın veya başka bir örnek yolu ve çıktı adı verin.

<a name="goal-notation-and-obstacles"></a>

#### Hedef gösterimi ve engeller

Kalın çerçeve yaprağı, yamuk betimleyici ifadeyi gösterir. Ters eğimli risk şekli, engel çizgisi üzerinden hedefe yönelir. Açıklama bulut içindedir. [Drizzle kaynağı](../../sample/notation.zz).

![Bulut, yamuk, altıgen ve enine çizgi içeren hedef ve risk gösterimi](../../docs/images/notation.png)

<a name="requirement-refinement"></a>

#### Gereksinimlerin ayrıntılandırılması

Bağımsız bağlantılar, ayrıntılandırılan gereksinime ve yapraklarına ayrı işaretler ve ok uçları verir. [Drizzle kaynağı](../../sample/requirements.zz).

![Kalın çerçeveli üç yaprağı olan aşağıdan yukarı ayrıntılandırma ağacı](../../docs/images/requirements.png)

<a name="rail-goal-conflicts"></a>

#### Demiryolu hedef çatışmaları

Dallar birden çok alt hedefi tek birleşimde toplar. Zikzaklar ağaçtaki çatışan hedefleri bağlar. [Drizzle kaynağı](../../sample/rail-conflicts.zz).

![Ortak daireli birleşimler ve iki zikzak çatışması olan demiryolu hedef diyagramı](../../docs/images/rail-conflicts.png)

<a name="authentication-risk"></a>

#### Kimlik doğrulama riski

Ters eğimli şekiller oturum açmayı engelleyen sorunları gösterir. Dıştaki zikzak kurtarma seçeneğinin oluşturabileceği çatışmayı belirtir. [Drizzle kaynağı](../../sample/auth-risk.zz).

![Sağ kenardan dolaşan zikzak rotalı kimlik doğrulama risk ağacı](../../docs/images/auth-risk.png)

<a name="de-morgan-goal-and-risk-diagram"></a>

#### De Morgan hedef ve risk diyagramı

Kaynak hedefi ve değillemesini, ortak ayrıntılandırma dalını ve üç kesikli engeli açıkça tanımlar. [Drizzle kaynağı](../../sample/de-morgan.zz).

![Ortak dallı ve kesikli engel çizgili hedef ve değillenmiş hedef](../../docs/images/de-morgan.png)

<a name="agent-monitoring-and-control"></a>

#### Aktörün izleme ve kontrol işlevi

Altıgen aktör ilk nesneden bir öznitelik alır ve ikinci nesnenin bir özniteliğini kontrol eder. [Drizzle kaynağı](../../sample/object-agent.zz).

![Giriş nesnesi aktöre, aktör çıkış nesnesine bağlanır](../../docs/images/object-agent.png)

<a name="train-control"></a>

#### Tren kontrolü

Düz ve kesikli çizgiler, ölçüm ve komutlar için ayrı oklar, eylem elipsi ve sorumluluk notu tek diyagramdadır. [Drizzle kaynağı](../../sample.zz).

![Aktör, nesne, eylem ve sorumluluk notu içeren tren kontrol gereksinimleri](../../output.png)

<a name="editor-and-diagnostics"></a>

### Düzenleyici ve tanılar

`drizzle/` içindeki VS Code uzantısı `.zz`, `.drizzle` ve `.rd` dosyalarını vurgular. Güncel dilbilgisini denemek için bu klasörü VS Code'da açıp **F5** ile Extension Development Host'u başlatın. Diyagram kaynağını bu pencerede açın. [VS Code uzantı belgeleri](https://code.visualstudio.com/api/get-started/your-first-extension).

| İleti veya belirti | Çözüm |
| --- | --- |
| `unknown node 'X'` | `X` düğümünü aynı harf büyüklüğüyle tanımlayın veya başvuruyu düzeltin. |
| `duplicate ID` | Her tanıma benzersiz bir düğüm kimliği verin. |
| `cell @… is already occupied` | Düğümlerden birini başka ızgara hücresine atayın. |
| `invalid Drizzle syntax` | İletideki dosya, satır ve sütunla hatalı ayırıcıyı, işleci veya seçeneği bulun. |
| `diagram is too large for PNG output` | Uzak ızgara konumlarını yaklaştırın veya modeli birden fazla diyagrama bölün. |
| `no system fonts found` | Bir sans-serif yazı tipi kurup derleyiciyi yeniden çalıştırın. |
| `link.exe not found` | MSVC derleme araçlarını yükleyin veya yukarıdaki GNU komutunu çalıştırın. |

<a name="installation"></a>

## Kurulum

Aşağıdaki komutlar, iş akışı dosyaları yükledikten sonra `lmdred-v1.2.0` derleyici sürümünü indirir.

Kurucular x64 veya ARM64 mimarisini seçer, arşivi ve SHA-256 toplamını indirip doğrular ve `lmdred` aracını kullanıcı hesabına kurar. Yükseltmek veya kurulumu onarmak için kurucuyu yeniden çalıştırın.

<a name="linux-and-macos"></a>

### Linux ve macOS

Kabuk kurucusunu curl ile indirip çalıştırın.

```sh
curl -fL https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.sh -o install-lmdred.sh
sh install-lmdred.sh
```

wget ile de indirebilirsiniz.

```sh
wget https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.sh -O install-lmdred.sh
sh install-lmdred.sh
```

Varsayılan dizin `$HOME/.local/bin` konumudur. Kurucu `~/.profile` dosyasını ve kullanılan Bash, Zsh veya Fish kabuğunun başlangıç dosyalarını günceller. Kurulum sonrası yeni bir terminal açın. Bash, Zsh veya sh içinde komutu mevcut terminalde de etkinleştirebilirsiniz.

```sh
export PATH="$HOME/.local/bin:$PATH"
lmdred --version
```

Sürümü veya kurulum dizinini aşağıdaki seçeneklerle belirleyin.

```sh
sh install-lmdred.sh --version 1.2.0 --bin-dir "$HOME/.local/bin"
sh install-lmdred.sh --no-modify-path
```

`LMDRED_VERSION` ve `LMDRED_INSTALL_DIR` ortam değişkenleri varsayılanları belirler. Açıkça verilen komut satırı seçenekleri önceliklidir. Başka bir kabukta kurulum dizinini başlangıç yapılandırmasına ekleyin.

<a name="windows"></a>

### Windows

Kurucuyu PowerShell'den indirip çalıştırın.

```powershell
$installer = Join-Path $env:TEMP 'install-lmdred.ps1'
Invoke-WebRequest -Uri 'https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.ps1' -OutFile $installer
powershell -NoProfile -ExecutionPolicy Bypass -File $installer
```

Yeni terminal açıp `lmdred --version` çalıştırın. Kurucu yürütülebilir dosyayı `%LOCALAPPDATA%\Programs\lmdred\bin` altına koyar ve dizini kullanıcı PATH'inin başına ekler. Böylece Cargo ile kurulu kopyadan önce gelir. Makine genelindeki kopya öncelik kazanırsa kurucu yolunu bildirir. Terminalin hangi kopyayı seçtiğini `Get-Command lmdred -All` ile kontrol edin.

Farklı konum seçmek veya PATH'i kendiniz yönetmek için bu seçenekleri kullanın.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -Version 1.2.0 -InstallDirectory "$env:LOCALAPPDATA\Programs\lmdred\bin"
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -NoModifyPath
```

`LMDRED_VERSION` ve `LMDRED_INSTALL_DIR` Windows'ta da varsayılanları belirler.

<a name="download-an-archive-directly"></a>

### Arşivi doğrudan indirme

Sisteminize uygun arşivi açın ve yürütülebilir dosyayı PATH'teki bir dizine koyun.

| Sistem | İşlemci | Sürüm arşivi |
| --- | --- | --- |
| Windows | x64 | `lmdred-v1.2.0-x86_64-pc-windows-msvc.zip` |
| Windows | ARM64 | `lmdred-v1.2.0-aarch64-pc-windows-msvc.zip` |
| macOS | Intel | `lmdred-v1.2.0-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon | `lmdred-v1.2.0-aarch64-apple-darwin.tar.gz` |
| Linux | x64 | `lmdred-v1.2.0-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 | `lmdred-v1.2.0-aarch64-unknown-linux-musl.tar.gz` |

Linux ikilileri glibc ve musl dağıtımlarını desteklemek için musl kullanır. Windows sürümleri C çalışma zamanı kitaplığına statik bağlanır. Diyagram oluşturma sistem yazı tiplerini kullanır. Asgari Linux sunucularında veya kapsayıcılarında DejaVu Sans gibi bir sans-serif yazı tipi kurun.

<a name="install-from-source"></a>

### Kaynaktan kurma

Rust ve yerel derleme araçlarını kurduktan sonra şu komutu depo kökünden çalıştırın.

```sh
cargo install --path . --locked
```

Cargo `lmdred` komutunu `~/.cargo/bin` dizinine, Windows'ta ise `%USERPROFILE%\.cargo\bin` dizinine kurar. Gerekirse dizini PATH'e ekleyin. GitHub kaynağından doğrudan da kurabilirsiniz.

```sh
cargo install --git https://github.com/Asncodes-80/lamda_red --locked --bin lmdred
```

Windows GNU araç zinciri için bu komutu kullanın.

```powershell
cargo +stable-x86_64-pc-windows-gnu install --path . --locked
```

<a name="source-and-development"></a>

## Kaynak kod ve geliştirme

Derleyici UTF-8 kaynak dosyasını okur, düğümleri ve bağlantıları ayrıştırır, etiketleri ölçer, yerleşimi ve rotaları hesaplar ve diyagramı dışa aktarır.

| Konum | Görev |
| --- | --- |
| [src/main.rs](../../src/main.rs) | Komut satırı argümanları, dosya yolları, hatalar ve derleme aşamaları |
| [src/lamda_red_grammar.pest](../../src/lamda_red_grammar.pest) | Drizzle dilbilgisi |
| [src/parsing.rs](../../src/parsing.rs) | Diyagram modeli, tanımlar, başvurular ve sözdizimi tanıları |
| [src/labels.rs](../../src/labels.rs) | Yazı tipi bulma, metin biçimleme, satır kaydırma ve kısaltma |
| [src/layout.rs](../../src/layout.rs) | Şekil geometrisi, ızgara yerleşimi ve tuval sınırları |
| [src/routing.rs](../../src/routing.rs) | Bağlantılar, dallar, dış rotalar ve bağlantı etiketi konumları |
| [src/export.rs](../../src/export.rs) | PNG ve mxGraph XML üretimi |
| [sample.zz](../../sample.zz) ve [sample/](../../sample/) | Tren kontrolü ve diğer referans diyagramları |
| [tests/test_cli_install.py](../../tests/test_cli_install.py) | CLI ve sürüm kurucularının bütünleştirme testleri |
| [scripts/](../../scripts/) | Kurucular ve sürüm arşivlerinin oluşturulması |
| [.github/workflows/release-cli.yml](../../.github/workflows/release-cli.yml) | Altı hedef için derleme, test ve yayın |
| [drizzle/](../../drizzle/) | VS Code sözdizimi vurgulama uzantısı |

<a name="set-up-the-source"></a>

### Kaynak kodu hazırlama

Güncel ve kararlı bir Rust araç zincirini, Cargo'yu ve platformun yerel derleme araçlarını kurun. CLI ve kurucu bütünleştirme testlerinde Python 3 kullanılır. Sürüm paketleyicisi Windows PowerShell 5.1 veya PowerShell 7'de çalışır. PNG oluşturucu sistemde kurulu yazı tiplerini kullanır. [Rust kurulumu](https://www.rust-lang.org/tools/install).

```sh
git clone https://github.com/Asncodes-80/lamda_red.git
cd lamda_red
cargo build --locked
```

Windows'ta MSVC derlemeleri için Visual Studio C++ derleme araçları gerekir. macOS'ta Xcode komut satırı araçlarını, Linux'ta bir C derleyicisi ve bağlayıcı kurun. Asgari Linux ortamlarında DejaVu Sans gibi bir yazı tipi de gerekir.

Windows GNU araç zinciri için aşağıdaki komutları kullanın. Bu kılavuzdaki diğer derleme, çalıştırma, test ve kurulum komutlarında `+stable-x86_64-pc-windows-gnu` seçeneğini `cargo` komutunun hemen sonuna ekleyebilirsiniz.

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal --component clippy,rustfmt
cargo +stable-x86_64-pc-windows-gnu build --locked
```

`Cargo.lock` dosyasını depoda tutun ve kaydedilmiş bağımlılık sürümleri için `--locked` kullanın.

<a name="run-and-debug"></a>

### Çalıştırma ve hata ayıklama

Cargo'nun geliştirme derlemesi hata ayıklama bilgisi içerir. Aşağıdaki komutla bir referans diyagramı derleyin.

```sh
cargo run --locked -- sample/requirements.zz target/requirements
cargo run --locked -- --help
cargo run --locked -- --version
```

İlk komut `target/requirements.png` ve `target/requirements.xml` dosyalarını yazar. `cargo run --locked` tek başına çalıştırıldığında `sample.zz` dosyasını okur ve depo kökünde `output.png` ve `output.xml` üretir.

VS Code'da hata ayıklamak için depo kökünü açın, CodeLLDB'yi kurun ve aşağıdaki yapılandırmayı `.vscode/launch.json` dosyasına ekleyin. `lmdred`, Cargo ile derlenir ve örnek, depo köküne göreli yollarla açılır. [CodeLLDB yapılandırma belgesi](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#cargo-support).

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

`run` işlevine, `src/main.rs` dosyasında bir kesme noktası koyun ve **F5** tuşuna basın. Modeli incelemek için `parsing::parse`, konumları ve rotaları görmek için `layout::arrange`, çıktıyı incelemek için `export::png` veya `export::xml` içine adımlayın. GNU araç zincirini açıkça seçmek için Cargo argümanlarını `["+stable-x86_64-pc-windows-gnu", "build", "--locked", "--bin", "lmdred"]` olarak değiştirin.

<a name="test-and-check"></a>

### Test ve denetim

Rust testlerini, biçimlendiriciyi ve linter'ı depo kökünden çalıştırın.

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

Rust testleri ayrıştırmayı ve başvuruları, işaretleri ve dalları, geriye dönük uyumluluğu, engellerin çevresinden yönlendirmeyi ve metni kaydırıp kısaltmayı kapsar.

CLI ve kurucuları sınamak için bir sürüm ikilisi derleyin ve yolunu bütünleştirme testine verin.

```sh
cargo build --locked --release
python tests/test_cli_install.py --binary target/release/lmdred
```

Windows'ta `target/release/lmdred.exe` kullanın. Açıkça belirtilen bir Rust hedefi için `target/<target>/release/lmdred` veya `lmdred.exe` kullanın. Test depo dışından diyagram üretir, boşluklu yolları ve çıktı varsayılanlarını sınar ve arşivi yerel HTTP üzerinden sunar. Geçici dizinde kurulumu ve yeniden kurulumu çalıştırır, bozuk sağlama toplamının reddedildiğini ve kurulu dosyanın korunduğunu doğrular.

<a name="build-and-package"></a>

### Derleme ve paketleme

```sh
cargo build --locked
cargo build --locked --release
```

Geliştirme ikilileri `target/debug/`, eniyilenmiş ikililer `target/release/` dizinine yazılır. Yürütülebilir dosya Linux ve macOS'ta `lmdred`, Windows'ta `lmdred.exe` adını taşır.

Windows'ta hedefi derleyin ve Windows PowerShell 5.1 ile paketleyin. PowerShell 7 için `pwsh` komutunu `powershell.exe` yerine kullanın.

```powershell
$compilerTarget = ((rustc -vV | Select-String '^host: ').Line -split ': ', 2)[1]
cargo build --locked --release --target $compilerTarget
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\package-release.ps1 -Target $compilerTarget
```

Her iki adımda etkin Rust araç zincirinin hedefi kullanılır. x64 Windows için bu `x86_64-pc-windows-msvc` veya `x86_64-pc-windows-gnu` olur. Yürütülebilir dosya `target/<target>/release/` içindedir. Arşiv ve `.sha256` dosyası `target/dist/` altına yazılır. Her arşiv yürütülebilir dosyayı, lisansı, README'yi ve `sample.zz` dosyasını içerir. Unix hedefleri `.tar.gz` arşivlerini kullanır.

<a name="publish-a-compiler-release"></a>

### Derleyici sürümü yayımlama

Sürüm iş akışı Windows, macOS ve Linux'ta x64 ve ARM64 için yerel ikilileri derler ve test eder. Platform arşivlerini, SHA-256 toplamlarını, iki kurucuyu ve birleşik `SHA256SUMS` dosyasını yükler.

Kaynağı kaydedip uzak depoya gönderdikten sonra `Cargo.toml` ile eşleşen etiketi yayımlayın.

```sh
git tag lmdred-v1.2.0
git push origin lmdred-v1.2.0
```

Etiket iş akışını başlatır ve tüm derlemeler başarılı olduğunda derleyici sürümü oluşturulur. Aynı etiketle elle GitHub sürümü yayımlamak da iş akışını başlatır ve dosyaları o sürüme ekler. **Run workflow** düğmesi seçilen dal için indirilebilir Actions çıktıları oluşturur.

Sonraki sürümlerde eşleşen etiketi göndermeden önce `Cargo.toml` ve `Cargo.lock` dosyalarındaki derleyici sürümünü, iki kurucunun varsayılan sürümünü ve README'deki sürüme bağlı kurulum komutlarını güncelleyin. Derleyici etiketleri `lmdred-v` önekini taşır. VS Code uzantısı `drizzle-` etiketlerini ve `drizzle/package.json` içindeki kendi sürümünü kullanır.

<a name="roadmap"></a>

## Yol haritası

- Lamda RED Pest dilbilgisi için durum makinesi diyagramı oluşturmak.
- Rust sunucusundan PNG isteyen React arayüzlü bir web hizmeti geliştirmek.

<a name="license"></a>

## Lisans

Bu proje [GNU GPLv3](../../LICENSE) ile lisanslanmıştır.
