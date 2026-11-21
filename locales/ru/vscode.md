# Расширение Drizzle для VS Code

Расширение Drizzle подсвечивает синтаксис файлов `.zz`, `.drizzle` и `.rd` в VS Code.

![Drizzle](../../images/logo_drizzle_language.png)

## Демонстрация

![Drizzle in VS Code](../../images/drizzle_demo.png)

## Пример управления поездом

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

## Результат компиляции

![PNG](../../output.png)

Фигуры, маркеры связей и параметры раскладки описаны в [руководстве по синтаксису Drizzle](./README.md#drizzle-syntax).

## Разработка и проверка расширения

Откройте папку `drizzle/` в VS Code. В `package.json` описаны расширение и путь к грамматике. Файл `syntaxes/drizzle.tmLanguage.json` содержит грамматику TextMate, а `language-configuration.json` определяет комментарии и пары скобок.

Нажмите **F5**, чтобы открыть Extension Development Host. Откройте в нём файл примера и проверьте подсветку и настройки языка. После изменения источников перезапустите среду или перезагрузите её окно.

Установите расширение из Marketplace или установите пакет VSIX командой **Extensions: Install from VSIX...** в VS Code. Инструкции по разработке и публикации есть в [документации по расширениям VS Code](https://code.visualstudio.com/api).

## История версий расширения

### 1.1.3

- Исправлены адреса изображений логотипа, демонстрации синтаксиса и результата компиляции в Marketplace.
- Добавлена прямая ссылка на полное руководство Drizzle в README репозитория.

### 1.1.0 – 1.1.2

- Грамматика Pest поддерживает именованные узлы, ссылки вперёд, подписи UTF-8, экранирование, перенос строк и диагностику со строкой и столбцом. Сохранены анонимные цели, риски, агенты и однострочные комментарии.
- Добавлены объекты, действия, заметки, облачные примечания, описательные трапеции, ромбы и толстые контуры листьев, таких как требования и допущения.
- Добавлены сплошные и пунктирные связи, необязательные наконечники на одном или обоих концах, подписи, петли и общие узлы ветвления. Поддерживаются пустые и закрашенные круги, зигзаги конфликтов и поперечные черты препятствий.
- Добавлены четыре направления раскладки, явные позиции по строкам и столбцам, прямоугольные маршруты и внешние обходы. Параллельные связи разделяются, стрелки прикрепляются к контуру фигур.
- Обновлены грамматика VS Code и README расширения, демонстрация GIF заменена на PNG, добавлен VSIX Drizzle 1.1.0.

### 1.0.0

Первый выпуск Drizzle.
