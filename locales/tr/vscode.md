# VS Code için Drizzle uzantısı

Drizzle uzantısı VS Code'da `.zz`, `.drizzle` ve `.rd` dosyalarına sözdizimi vurgusu ekler.

![Drizzle](../../images/logo_drizzle_language.png)

## Tanıtım

![Drizzle in VS Code](../../images/drizzle_demo.png)

## Tren kontrolü örneği

```drizzle
&& Train control requirements
Estimate @0,0 = /Accurate estimate\nof speed & position/
Tracking @1,0 = <Tracking system>
Train @2,0 = [Train]
Duty @0,1 = {Train controller responsibility\nCompute safe acceleration for all trains between this station and the next.}
Safe @0,2 = /Safe command\nmessage/
Controller @1,2 = <Train controller>
Timing @0,3 = /Command in time/
Send @1,3 = (Send\ncommand)
Command @2,3 = [Command]

Estimate __ Tracking [o]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train __> Controller : As monitor\nmeasuredSpeed\nmeasuredLocation
Controller __> Command : As control\ncommandedSpeed\ncommandedAccel
Safe __ Controller [o]
Timing __ Controller [o] : Responsibility
Controller __ Send [o] : Performance
Duty --> Controller
```

## Derleyici çıktısı

![PNG](../../output.png)

Şekiller, bağlantı işaretleri ve yerleşim ayarları için [Drizzle sözdizimi kılavuzuna](./README.md#drizzle-syntax) bakın.

## Uzantıyı geliştirme ve sınama

`drizzle/` klasörünü VS Code'da açın. `package.json` uzantıyı ve dilbilgisi yolunu tanımlar. `syntaxes/drizzle.tmLanguage.json` TextMate dilbilgisini, `language-configuration.json` ise yorum ve ayraç ayarlarını içerir.

**F5** ile Extension Development Host penceresini açın. Bir örnek dosyayı açıp sözdizimi vurgusunu ve dil ayarlarını sınayın. Değişiklikten sonra ana bilgisayarı yeniden başlatın veya penceresini yeniden yükleyin.

Uzantıyı Marketplace'ten kurun veya VS Code'da **Extensions: Install from VSIX...** komutuyla bir VSIX paketi yükleyin. Geliştirme ve yayın için [VS Code uzantı belgelerine](https://code.visualstudio.com/api) bakın.

## Uzantı sürüm notları

### 1.1.3

- Marketplace'teki logo, sözdizimi gösterimi ve derlenmiş çıktı görsellerinin adresleri düzeltildi.
- Depo README'sindeki tam Drizzle kılavuzuna doğrudan bağlantı eklendi.

### 1.1.0 – 1.1.2

- Pest dilbilgisine adlandırılmış düğümler, ileri başvurular, UTF-8 etiketler, kaçışlar, satır sonları ve satır-sütun tanıları eklendi. Adsız hedef, risk ve aktör tanımları ile tam satır yorumları korundu.
- Nesneler, eylemler, notlar, bulut açıklamaları, betimleyici yamuklar ve eşkenar dörtgenler eklendi. Gereksinim ve varsayım gibi yapraklar kalın çerçeve kullanabilir.
- Düz ve kesikli bağlantılar, bir veya iki uçta isteğe bağlı ok uçları, etiketler, döngüler ve ortak birleşimler eklendi. Boş ve dolu daireler, çatışma zikzakları ve engel çizgileri desteklenir.
- Dört yerleşim yönü, açık sütun ve satır konumları, dik açılı rotalar ve dış rotalar eklendi. Paralel bağlantılar ayrılır ve oklar şekil sınırına bağlanır.
- VS Code dilbilgisi ve uzantı README'si güncellendi, GIF gösterimi PNG ile değiştirildi ve Drizzle 1.1.0 VSIX paketi eklendi.

### 1.0.0

Drizzle'ın ilk sürümü.
