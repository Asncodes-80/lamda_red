<div dir="rtl">

# إضافة Drizzle لـ VS Code

تلوّن إضافة Drizzle صياغة ملفات `.zz` و`.drizzle` و`.rd` في VS Code.

![Drizzle](../../images/logo_drizzle_language.png)

## عرض الإضافة

![Drizzle in VS Code](../../images/drizzle_demo.png)

## مثال للتحكم بالقطار

<div dir="ltr">

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

</div>

## إخراج المُصرّف

![PNG](../../output.png)

راجع [دليل صياغة Drizzle](./README.md#drizzle-syntax) لمعرفة الأشكال وعلامات الروابط وإعدادات التخطيط.

## تطوير الإضافة واختبارها

افتح مجلد `drizzle/` في VS Code. يحدّد `package.json` بيانات الإضافة وموقع قواعدها. يحتوي `syntaxes/drizzle.tmLanguage.json` على قواعد TextMate، ويضبط `language-configuration.json` التعليقات والأقواس.

اضغط **F5** لفتح Extension Development Host. افتح ملفًا نموذجيًا فيه واختبر تلوين الصياغة وإعدادات اللغة. بعد تعديل المصدر أعد تشغيل المضيف أو تحميل نافذته.

ثبّت الإضافة من Marketplace أو استخدم أمر **Extensions: Install from VSIX...** في VS Code لتثبيت حزمة VSIX. راجع [مستندات إضافات VS Code](https://code.visualstudio.com/api) لمعلومات التطوير والنشر.

## سجل إصدارات الإضافة

### 1.1.3

- تصحيح عناوين صور Marketplace للشعار وعرض الصياغة والمخطط الناتج.
- إضافة رابط مباشر إلى دليل Drizzle الكامل في README بالمستودع.

### 1.1.0 – 1.1.2

- توسيع قواعد Pest بالعقد المسماة والمراجع السابقة للتعريف وتسميات UTF-8 ومحارف الهروب وفواصل الأسطر ورسائل الخطأ التي تحدد السطر والعمود. استمرار دعم الأهداف والمخاطر والوكلاء بلا أسماء وتعليقات السطر الكامل.
- إضافة الكائنات والإجراءات والملاحظات والتعليقات السحابية وأشباه المنحرف والمعينات، ودعم الإطارات السميكة للأوراق مثل المتطلبات والافتراضات.
- إضافة الروابط المتصلة والمتقطعة ورؤوس الأسهم الاختيارية والتسميات والحلقات وتفرعات التفصيل المشتركة، مع دوائر فارغة وممتلئة وتعرجات للتعارض وشرطات عرضية للعوائق.
- إضافة أربعة اتجاهات للتخطيط وتحديد الصف والعمود ومسارات قائمة الزوايا والتفاف خارجي، مع فصل الروابط المتوازية وربط الأسهم بحدود الأشكال.
- تحديث قواعد إضافة VS Code وREADME الخاص بها، واستبدال عرض GIF بصورة PNG، وإضافة حزمة Drizzle 1.1.0 بصيغة VSIX.

### 1.0.0

الإصدار الأول من Drizzle.

</div>
