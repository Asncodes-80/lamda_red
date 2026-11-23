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
    <p>Diagramas de ingeniería de requisitos a partir de texto plano</p>
</div>

Lamda RED convierte archivos de texto `.zz` en diagramas de requisitos. Úsalo para mostrar qué debe lograr un sistema, qué riesgos pueden impedirlo y cómo interactúan los agentes y objetos. Guarda el código fuente junto a los requisitos en Git para que el equipo pueda revisar los cambios en el diff y regenerar las imágenes.

Escribe el diagrama en **Drizzle** y compílalo con **`lmdred`**. El compilador genera un PNG para documentos y XML de mxGraph editable para diagrams.net (draw.io). Con `--export-set` obtienes tres PNG, uno compacto, uno para documentos y uno de alta resolución. La extensión para VS Code de escritorio muestra el diagrama junto al editor y lo exporta al guardar o al pulsar **Export diagram**.

## De Drizzle al diagrama

En este ejemplo, un agente supervisa un valor de un objeto y controla otro. La imagen muestra el resultado.

![Un objeto de entrada se conecta a un agente, que se conecta al objeto de salida](../../docs/images/object-agent.png)

Guarda este ejemplo como `object-agent.zz`.

```drizzle
Input = [Object (obj1)]
Monitor = <Agent (Ag)>
Output = [Object (obj2)]

Input __> Monitor : Obj1.attribute1\nValue monitoring
Monitor __> Output : Obj2.attribute2\nState variable\nControl
```

Tras instalar el compilador, ejecuta el comando en la carpeta del archivo. Se generan `object-agent.png` y `object-agent.xml`.

```sh
lmdred object-agent.zz object-agent
```

[Instalación](#installation) · [Vista previa en VS Code](../../README.md#live-preview-in-vs-code) · [Guía de sintaxis de Drizzle](#drizzle-syntax)

- [Uso de la CLI](#cli-usage)
- [Sintaxis de Drizzle](#drizzle-syntax)
- [Extensión Drizzle para VS Code](./vscode.md)
- [Instalación](#installation)
- [Código fuente y desarrollo](#source-and-development)
  - [Preparar el código fuente](#set-up-the-source)
  - [Ejecutar y depurar](#run-and-debug)
  - [Pruebas y comprobaciones](#test-and-check)
  - [Compilar y empaquetar](#build-and-package)
  - [Publicar una versión del compilador](#publish-a-compiler-release)
- [Guía de desarrollo](../../DEVELOPER.md)
- [Guía de contribución](../../CONTRIBUTING.md)
- [Código de conducta](../../CODE_OF_CONDUCT.md)
- [Registro de cambios](./CHANGELOG.md)
- [Hoja de ruta](#roadmap)
- [Licencia](#license)

<a name="cli-usage"></a>

## Uso de la CLI

Ejecuta el comando instalado desde cualquier directorio.

```text
lmdred [input.zz] [output-stem]
lmdred --help
lmdred --version
```

| Argumento u opción | Significado | Predeterminado |
| --- | --- | --- |
| `input.zz` | Ruta a un archivo fuente Drizzle en UTF-8 | `sample.zz` en el directorio actual |
| `output-stem` | Ruta de salida antes de añadir las extensiones `.png` y `.xml` | `output` en el directorio actual |
| `-h`, `--help` | Mostrar la ayuda de la CLI |  |
| `-V`, `--version` | Mostrar la versión del compilador, por ejemplo `lmdred 1.2.0` |  |
| `--` | Tratar los argumentos restantes como rutas de archivos |  |

<a name="output-paths"></a>

### Rutas de salida

Las rutas relativas se resuelven desde el directorio de trabajo del terminal. Usa una ruta absoluta para elegir otra ubicación. El directorio de salida debe existir. El compilador añade las extensiones al nombre base indicado.

| Comando | Archivos escritos con respecto al directorio actual |
| --- | --- |
| `lmdred sample.zz my-new-red` | `my-new-red.png` y `my-new-red.xml` |
| `lmdred sample.zz` | `output.png` y `output.xml` |
| `lmdred` | Lee `sample.zz` y escribe `output.png` y `output.xml` |
| `lmdred diagrams/train.zz` | `output.png` y `output.xml` |
| `lmdred diagrams/train.zz reports/train` | `reports/train.png` y `reports/train.xml` |
| `lmdred sample.zz train.v1` | `train.v1.png` y `train.v1.xml` |

Pon entre comillas las rutas que contengan espacios. Usa `--` cuando una ruta empiece por `-`.

```sh
lmdred "my diagrams/train.zz" "my diagrams/train"
lmdred -- "-file.zz" result
```

<a name="output-formats-and-errors"></a>

### Formatos de salida y errores

El lienzo del PNG contiene las figuras, conexiones y etiquetas. El XML de mxGraph incluye figuras, conexiones y puntos de paso editables en diagrams.net/draw.io.

Una compilación correcta muestra las dos rutas de salida y devuelve el estado `0`. Los fallos emiten un mensaje en el flujo de error estándar y devuelven `1`. Los errores de sintaxis y referencia indican archivo, línea y columna.

Al guardar correctamente se sobrescriben las salidas ya existentes. El compilador comprueba que las rutas de salida no sustituyan al archivo fuente. Analiza y genera el diagrama antes de escribir. Un error del sistema de archivos durante el guardado puede dejar una sola salida escrita. Revisa el mensaje y vuelve a ejecutar tras corregir la ruta o los permisos.

<a name="drizzle-syntax"></a>

## Sintaxis de Drizzle

Usa el PNG en documentos y presentaciones. Guarda el archivo Drizzle con los requisitos para que quienes los revisen puedan seguir y modificar objetivos, riesgos, agentes y relaciones. Cada compilación genera también XML de mxGraph editable.

<a name="a-first-diagram"></a>

### Primer diagrama

Guarda este ejemplo como `first.zz` y ejecuta `lmdred first.zz first`.

```drizzle
&& The tracking agent reports train speed and location.
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
```

La salida tiene un agente hexagonal, un objeto rectangular y una flecha continua hacia el tren. La etiqueta sobre la flecha ocupa dos líneas.

<a name="declare-nodes"></a>

### Declarar nodos

Usa `ID = shape` para que las conexiones puedan referirse al nodo. Los ID distinguen mayúsculas y minúsculas, empiezan con una letra ASCII y admiten letras, números y guiones bajos simples entre caracteres alfanuméricos. `Train_1` es un ID. El texto dentro de la figura es su etiqueta visible. Las conexiones pueden ir antes o después de las declaraciones. Escribe `&&` al inicio de la línea para un comentario. Se permiten líneas en blanco y sangría.

También puedes escribir figuras por separado, como `/Safe journey/`. Asigna un ID para usar una figura en una conexión.

| Figura | Declaración | Resultado |
| --- | --- | --- |
| Objetivo | `G = /Safe journey/` | Paralelogramo inclinado a la derecha |
| Riesgo | `R = \Signal lost\` | Paralelogramo reflejado |
| Agente | `A = <Train controller>` | Hexágono |
| Enunciado descriptivo | `D = ~Doors are locked in motion~` | Trapecio con base más ancha |
| Objeto | `O = [Train]` | Rectángulo |
| Acción | `Send = (Send command)` | Elipse |
| Nota | `N = {Controller responsibility}` | Rectángulo verde claro con borde discontinuo |
| Anotación en nube | `C = {{Reason for the obstacle}}` | Contorno de nube |
| Rombo | `Check = ^Speed confirmed?^` | Rombo |

Antepón `!` para engrosar el contorno. La notación de referencia lo utiliza para hojas como los requisitos y los supuestos. La etiqueta conserva su grosor normal.

```drizzle
/Goal/
!/Leaf requirement/
~Descriptive~
<Agent 1>
```

<a name="labels-and-line-breaks"></a>

#### Etiquetas y saltos de línea

Las etiquetas aceptan UTF-8, incluido texto de derecha a izquierda. El generador mide y centra el texto y adapta la figura. Las etiquetas largas se dividen entre palabras cuando es posible y dentro de palabras demasiado largas. Se muestran hasta cuatro líneas por figura y tres por conexión. El exceso termina en `...`. El origen conserva la etiqueta completa.

| Sintaxis | Resultado |
| --- | --- |
| `/Accurate estimate\nof speed and position/` | Etiqueta de objetivo con salto de línea |
| `[C:\\Logs]` | Rectángulo con etiqueta `C:\Logs` |
| `<Speed \> zero>` | Agente con etiqueta `Speed > zero` |
| `/Input \/ output/` | Objetivo con etiqueta `Input / output` |
| `~Value \~ threshold~` | Trapecio con etiqueta `Value ~ threshold` |

Escribe `\n` para un salto forzado, `\\` para una barra inversa literal y una barra inversa antes de un delimitador que quieras mostrar dentro de la etiqueta.

<a name="connect-nodes"></a>

### Conectar nodos

Usa ID de nodo a ambos lados del operador. Los dos caracteres centrales determinan el estilo de línea. Los símbolos exteriores `<` y `>` añaden puntas de flecha a la izquierda y a la derecha, respectivamente. Puedes separar los ID de los operadores con espacios.

| Dirección | Continua | Discontinua |
| --- | --- | --- |
| Sin puntas de flecha | `A __ B` | `A -- B` |
| Hacia B | `A__>B` | `A-->B` |
| Hacia A | `A<__B` | `A<--B` |
| Hacia ambos extremos | `A<__>B` | `A<-->B` |

Añade dos puntos después de la conexión y sus opciones para escribir la etiqueta. Dos conexiones entre los mismos nodos pueden llevar etiquetas distintas y seguir rutas separadas. Una conexión repetida se dibuja una sola vez. Una conexión de un nodo a sí mismo forma un bucle.

```drizzle
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train --> Train : Refresh state
```

<a name="add-notation-to-a-connection"></a>

#### Añadir marcas a una conexión

| Opción | Ejemplo | Resultado |
| --- | --- | --- |
| `[o]` | `A __ B [o]` | Círculo vacío en la línea |
| `[*]` | `A __> B [*]` | Círculo relleno en la línea |
| `[zigzag]` | `A __ B [zigzag]` | Zigzag para un conflicto |
| `[bar]` | `Risk --> Goal [bar]` | Barra transversal para un obstáculo |
| `[ortho]` | `A __> B [ortho]` | Ruta con tramos horizontales y verticales |

Usa un solo círculo, zigzag o barra transversal por conexión. La marca se puede combinar con `[ortho]` y cualquiera de los estilos de línea. En una conexión con una flecha la barra queda cerca de la punta. Sin flechas o con dos, queda cerca de la mitad de la ruta.

<a name="join-several-sources"></a>

#### Unir varios orígenes

Las conexiones dirigidas al mismo destino comparten el tramo final si coinciden el estilo, el marcador, la etiqueta y las opciones de ruta. Los desvíos explícitos conservan sus rutas. Separa los ID de origen con comas para declarar una conexión compartida. El círculo vacío o relleno queda en la unión, la punta derecha va hacia el destino y la izquierda apunta a cada origen de una rama explícita.

```drizzle
layout up
Goal @1,0 = /Safe departure/
Software @0,1 = !/Verify the lock signal/
Assumption @2,1 = !/The door sensor works/

Software, Assumption __> Goal [o]
```

Los dos nodos hoja confluyen en una línea con un círculo vacío y una sola flecha al objetivo superior. Escribir `Software __> Goal [o]` y `Assumption __> Goal [o]` en líneas separadas da el mismo resultado.

<a name="set-the-layout-and-routing"></a>

### Definir la disposición y las rutas

| Control | Ejemplo | Resultado |
| --- | --- | --- |
| Dirección | `layout right` | Flujo automático de izquierda a derecha, el predeterminado |
| Dirección | `layout left` | Flujo automático de derecha a izquierda |
| Dirección | `layout up` | Flujo automático de abajo a arriba con rutas ortogonales |
| Dirección | `layout down` | Flujo automático de arriba a abajo con rutas ortogonales |
| Ubicación en cuadrícula | `Goal @2,0 = /Goal/` | Coloca la figura en la columna 2, fila 0 |
| Ruta exterior | `A --> B [bar] via bottom` | Lleva la conexión por debajo del diagrama |

Escribe como máximo una directiva de disposición por diagrama. Las coordenadas empiezan en cero, con columnas hacia la derecha y filas hacia abajo. Cada celda explícita admite un nodo. Lamda RED dimensiona filas y columnas según las figuras y ajusta el lienzo PNG al diagrama. Se puede combinar la ubicación automática y la explícita.

La figura sitúa el ejemplo de objetos y agente en la fila cero. Las guías punteadas y los puntos de cruce ilustran las coordenadas. La intersección resaltada `@0,2` está en la columna cero, fila dos.

![Diagrama de objetos y agente sobre una cuadrícula punteada numerada desde cero](../../docs/images/grid-placement.png)

```drizzle
Input @0,0 = [Object (obj1)]
Monitor @1,0 = <Agent (Ag)>
Output @2,0 = [Object (obj2)]

Input __> Monitor : Obj1.attribute1\nValue monitoring
Monitor __> Output : Obj2.attribute2\nState variable\nControl
```

Para añadir una nota en la intersección resaltada, escribe `Explanation @0,2 = {Column zero, row two}`. Las coordenadas seleccionan una celda y su posición en píxeles depende de las dimensiones de las figuras.

Escribe `via right`, `via left`, `via top` o `via bottom` después de las opciones de conexión y antes de la etiqueta. Así un conflicto u obstáculo puede rodear otras relaciones.

```drizzle
layout up
Login @0,0 = /User can sign in/
Blocked @0,1 = \User cannot sign in\
Recover @0,2 = /Recover using a phone number/

Recover __> Blocked
Blocked __> Login
Recover __> Login [zigzag] via right : A non-user might gain access
```

Las dos primeras conexiones apuntan hacia arriba en el árbol. El conflicto rodea el árbol por la derecha y vuelve hacia el objetivo.

<a name="reference-diagrams"></a>

### Diagramas de referencia

Cada ejemplo muestra el resultado de compilar el archivo Drizzle enlazado. Ejecuta `lmdred sample/de-morgan.zz de-morgan` para el diagrama de De Morgan o usa otra ruta de ejemplo y nombre base.

<a name="goal-notation-and-obstacles"></a>

#### Notación de objetivos y obstáculos

El borde grueso distingue una hoja y el trapecio registra un enunciado descriptivo. La forma reflejada de riesgo apunta a un objetivo a través de una barra de obstáculo. Una nube contiene la explicación. [Código Drizzle](../../sample/notation.zz).

![Notación de objetivos y riesgos con nube, trapecio, hexágono y barra transversal](../../docs/images/notation.png)

<a name="requirement-refinement"></a>

#### Refinamiento de requisitos

Las conexiones independientes permiten que el requisito refinado y las hojas tengan sus propios marcadores y puntas de flecha. [Código Drizzle](../../sample/requirements.zz).

![Árbol ascendente de refinamiento con tres hojas de borde grueso](../../docs/images/requirements.png)

<a name="rail-goal-conflicts"></a>

#### Conflictos de objetivos ferroviarios

Las ramas reúnen varios refinamientos en una unión. Los zigzags conectan objetivos en conflicto a través del árbol. [Código Drizzle](../../sample/rail-conflicts.zz).

![Diagrama ferroviario con uniones circulares y dos conflictos en zigzag](../../docs/images/rail-conflicts.png)

<a name="authentication-risk"></a>

#### Riesgo de autenticación

Las figuras reflejadas muestran problemas que impiden iniciar sesión. El zigzag exterior indica el conflicto que podría introducir una opción de recuperación. [Código Drizzle](../../sample/auth-risk.zz).

![Árbol de riesgos de autenticación con una ruta en zigzag por la derecha](../../docs/images/auth-risk.png)

<a name="de-morgan-goal-and-risk-diagram"></a>

#### Diagrama de objetivos y riesgos de De Morgan

El origen declara explícitamente el objetivo y su negación, la rama compartida y los tres obstáculos de línea discontinua. [Código Drizzle](../../sample/de-morgan.zz).

![Objetivo y objetivo negado con rama compartida y relaciones discontinuas con barra transversal](../../docs/images/de-morgan.png)

<a name="agent-monitoring-and-control"></a>

#### Supervisión y control mediante un agente

El agente hexagonal recibe un atributo del primer objeto y controla un atributo del segundo. [Código Drizzle](../../sample/object-agent.zz).

![Un objeto de entrada conecta con el agente, que conecta con un objeto de salida](../../docs/images/object-agent.png)

<a name="train-control"></a>

#### Control de trenes

Un mismo diagrama reúne líneas continuas y discontinuas, flechas separadas para mediciones y órdenes, una elipse de acción y una nota de responsabilidad. [Código Drizzle](../../sample.zz).

![Requisitos de control de trenes con agentes, objetos, una acción y una nota de responsabilidad](../../output.png)

<a name="editor-and-diagnostics"></a>

### Editor y diagnóstico

La extensión de VS Code en `drizzle/` resalta archivos `.zz`, `.drizzle` y `.rd`. Para probar la gramática actual, abre esa carpeta en VS Code y pulsa **F5** para iniciar Extension Development Host. Abre el origen del diagrama en la ventana del anfitrión. [Documentación de extensiones de VS Code](https://code.visualstudio.com/api/get-started/your-first-extension).

| Mensaje o síntoma | Solución |
| --- | --- |
| `unknown node 'X'` | Declara `X` con las mismas mayúsculas y minúsculas o corrige la referencia. |
| `duplicate ID` | Asigna un ID único a cada nodo declarado. |
| `cell @… is already occupied` | Asigna una celda distinta a uno de los nodos. |
| `invalid Drizzle syntax` | Usa el archivo, la línea y la columna del mensaje para localizar el delimitador, operador u opción incorrectos. |
| `diagram is too large for PNG output` | Acerca las posiciones dispersas de la cuadrícula o reparte el modelo en varios diagramas. |
| `no system fonts found` | Instala una fuente sin serifa y vuelve a ejecutar el compilador. |
| `link.exe not found` | Instala las herramientas de compilación de MSVC o ejecuta el comando GNU anterior. |

<a name="installation"></a>

## Instalación

Los siguientes comandos descargan la versión `lmdred-v1.2.0` del compilador después de que el flujo de publicación suba los archivos.

Los instaladores seleccionan x64 o ARM64, descargan el archivo y su suma SHA-256, verifican el archivo e instalan `lmdred` en tu cuenta. Ejecuta de nuevo el instalador para actualizar o reparar la instalación.

<a name="linux-and-macos"></a>

### Linux y macOS

Descarga y ejecuta el instalador de shell con curl.

```sh
curl -fL https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.sh -o install-lmdred.sh
sh install-lmdred.sh
```

También puedes descargarlo con wget.

```sh
wget https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.sh -O install-lmdred.sh
sh install-lmdred.sh
```

El directorio predeterminado es `$HOME/.local/bin`. El instalador actualiza `~/.profile` y los archivos de inicio del shell actual, Bash, Zsh o Fish. Abre un terminal nuevo tras instalar. En Bash, Zsh o sh también puedes activar el comando en el terminal actual.

```sh
export PATH="$HOME/.local/bin:$PATH"
lmdred --version
```

Selecciona una versión o directorio con estas opciones.

```sh
sh install-lmdred.sh --version 1.2.0 --bin-dir "$HOME/.local/bin"
sh install-lmdred.sh --no-modify-path
```

Las variables de entorno `LMDRED_VERSION` y `LMDRED_INSTALL_DIR` proporcionan los valores predeterminados. Las opciones explícitas tienen prioridad. Para otros shells, añade el directorio a la configuración de inicio.

<a name="windows"></a>

### Windows

Descarga y ejecuta el instalador desde PowerShell.

```powershell
$installer = Join-Path $env:TEMP 'install-lmdred.ps1'
Invoke-WebRequest -Uri 'https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.ps1' -OutFile $installer
powershell -NoProfile -ExecutionPolicy Bypass -File $installer
```

Abre un terminal nuevo y ejecuta `lmdred --version`. El instalador coloca el ejecutable en `%LOCALAPPDATA%\Programs\lmdred\bin` y añade ese directorio al principio del PATH del usuario. Así tiene prioridad sobre una copia instalada con Cargo. Si se usa primero una copia de todo el sistema, se informa de su ruta. Usa `Get-Command lmdred -All` para comprobar qué copia selecciona el terminal.

Usa estas opciones para instalar en otra ubicación o gestionar el PATH manualmente.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -Version 1.2.0 -InstallDirectory "$env:LOCALAPPDATA\Programs\lmdred\bin"
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -NoModifyPath
```

`LMDRED_VERSION` y `LMDRED_INSTALL_DIR` también definen los valores predeterminados en Windows.

<a name="download-an-archive-directly"></a>

### Descargar un archivo directamente

Extrae el archivo de tu sistema y coloca el ejecutable en un directorio del PATH.

| Sistema | Procesador | Archivo de distribución |
| --- | --- | --- |
| Windows | x64 | `lmdred-v1.2.0-x86_64-pc-windows-msvc.zip` |
| Windows | ARM64 | `lmdred-v1.2.0-aarch64-pc-windows-msvc.zip` |
| macOS | Intel | `lmdred-v1.2.0-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon | `lmdred-v1.2.0-aarch64-apple-darwin.tar.gz` |
| Linux | x64 | `lmdred-v1.2.0-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 | `lmdred-v1.2.0-aarch64-unknown-linux-musl.tar.gz` |

Los binarios de Linux usan musl para funcionar en distribuciones con glibc y musl. Las versiones de Windows enlazan estáticamente la biblioteca de ejecución de C. El renderizado usa fuentes del sistema. En servidores o contenedores Linux mínimos, instala una fuente sin serifa como DejaVu Sans.

<a name="install-from-source"></a>

### Instalar desde el código fuente

Con Rust y las herramientas de compilación instaladas, ejecuta lo siguiente en la raíz del repositorio.

```sh
cargo install --path . --locked
```

Cargo instala `lmdred` en `~/.cargo/bin`, o en `%USERPROFILE%\.cargo\bin` en Windows. Añade ese directorio al PATH si es necesario. También puedes instalar directamente desde el código fuente de GitHub.

```sh
cargo install --git https://github.com/Asncodes-80/lamda_red --locked --bin lmdred
```

Para la cadena de herramientas GNU en Windows, usa este comando.

```powershell
cargo +stable-x86_64-pc-windows-gnu install --path . --locked
```

<a name="source-and-development"></a>

## Código fuente y desarrollo

El compilador lee un archivo UTF-8, analiza los nodos y las conexiones, mide las etiquetas, calcula la disposición y las rutas, y exporta el diagrama.

| Ubicación | Responsabilidad |
| --- | --- |
| [src/main.rs](../../src/main.rs) | Argumentos de CLI, rutas de archivos, errores y proceso de compilación |
| [src/lamda_red_grammar.pest](../../src/lamda_red_grammar.pest) | Gramática de Drizzle |
| [src/parsing.rs](../../src/parsing.rs) | Modelo del diagrama, declaraciones, referencias y diagnóstico de sintaxis |
| [src/labels.rs](../../src/labels.rs) | Búsqueda de fuentes, composición, ajuste y truncamiento del texto |
| [src/layout.rs](../../src/layout.rs) | Geometría, ubicación en la cuadrícula y límites del lienzo |
| [src/routing.rs](../../src/routing.rs) | Conexiones, ramificaciones, desvíos y posición de etiquetas |
| [src/export.rs](../../src/export.rs) | Generación de PNG y XML de mxGraph |
| [sample.zz](../../sample.zz) y [sample/](../../sample/) | Control de trenes y otros diagramas de referencia |
| [tests/test_cli_install.py](../../tests/test_cli_install.py) | Pruebas de integración de la CLI y los instaladores |
| [scripts/](../../scripts/) | Instaladores y creación de archivos de distribución |
| [.github/workflows/release-cli.yml](../../.github/workflows/release-cli.yml) | Compilación, pruebas y publicaciones para seis destinos |
| [drizzle/](../../drizzle/) | Extensión de resaltado de sintaxis para VS Code |

<a name="set-up-the-source"></a>

### Preparar el código fuente

Instala una versión estable reciente de Rust, Cargo y las herramientas de compilación nativas. Las pruebas de integración requieren Python 3. La herramienta de empaquetado funciona en Windows PowerShell 5.1 o PowerShell 7. Para generar PNG se usan las fuentes instaladas en el sistema. [Instalación de Rust](https://www.rust-lang.org/tools/install).

```sh
git clone https://github.com/Asncodes-80/lamda_red.git
cd lamda_red
cargo build --locked
```

En Windows, la compilación con MSVC requiere las herramientas de compilación de Visual Studio C++. En macOS instala las herramientas de línea de comandos de Xcode. En Linux instala un compilador de C y un enlazador. En entornos Linux mínimos instala también una fuente como DejaVu Sans.

Para usar la cadena de herramientas GNU en Windows, ejecuta los siguientes comandos. Puedes insertar `+stable-x86_64-pc-windows-gnu` inmediatamente después de `cargo` en los demás comandos de compilación, ejecución, pruebas e instalación.

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal --component clippy,rustfmt
cargo +stable-x86_64-pc-windows-gnu build --locked
```

Mantén `Cargo.lock` en el repositorio y utiliza `--locked` para respetar las versiones registradas de las dependencias.

<a name="run-and-debug"></a>

### Ejecutar y depurar

La compilación de desarrollo de Cargo incluye información de depuración. Compila un diagrama de referencia con el siguiente comando.

```sh
cargo run --locked -- sample/requirements.zz target/requirements
cargo run --locked -- --help
cargo run --locked -- --version
```

El primer comando escribe `target/requirements.png` y `target/requirements.xml`. Ejecutar `cargo run --locked` sin más argumentos lee `sample.zz` y escribe `output.png` y `output.xml` en la raíz del repositorio.

Para depurar en VS Code, abre la raíz del repositorio, instala CodeLLDB y añade la siguiente configuración a `.vscode/launch.json`. Compila `lmdred` con Cargo y abre el ejemplo con rutas relativas al repositorio. [Referencia de configuración de CodeLLDB](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#cargo-support).

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

Pon un punto de interrupción en `run`, en `src/main.rs`, y pulsa **F5**. Entra en `parsing::parse` para inspeccionar el modelo, en `layout::arrange` para examinar posiciones y rutas, y en `export::png` o `export::xml` para revisar la salida. Para seleccionar explícitamente las herramientas GNU, cambia los argumentos de Cargo a `["+stable-x86_64-pc-windows-gnu", "build", "--locked", "--bin", "lmdred"]`.

<a name="test-and-check"></a>

### Pruebas y comprobaciones

Ejecuta las pruebas de Rust y las comprobaciones de formato y análisis estático desde la raíz del repositorio.

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

Las pruebas de Rust cubren el análisis, las referencias, los marcadores y ramas, la compatibilidad con versiones anteriores, la evasión de obstáculos, el ajuste de línea y el truncamiento de texto.

Para comprobar la CLI y los instaladores, compila un binario de distribución y pasa su ruta a la prueba de integración.

```sh
cargo build --locked --release
python tests/test_cli_install.py --binary target/release/lmdred
```

En Windows, usa `target/release/lmdred.exe`. Si eliges un destino de Rust explícito, usa `target/<target>/release/lmdred` o `lmdred.exe`. La prueba genera diagramas fuera del repositorio, verifica rutas con espacios y valores de salida predeterminados, y sirve un archivo de distribución por HTTP local. Instala y reinstala en un directorio temporal y comprueba que una suma incorrecta se rechace y el ejecutable instalado siga disponible.

<a name="build-and-package"></a>

### Compilar y empaquetar

```sh
cargo build --locked
cargo build --locked --release
```

Los binarios de desarrollo se escriben en `target/debug/` y los optimizados en `target/release/`. El ejecutable es `lmdred` en Linux y macOS y `lmdred.exe` en Windows.

En Windows, compila el destino y empaquétalo con Windows PowerShell 5.1. Si usas PowerShell 7, ejecuta `pwsh` en lugar de `powershell.exe`.

```powershell
$compilerTarget = ((rustc -vV | Select-String '^host: ').Line -split ': ', 2)[1]
cargo build --locked --release --target $compilerTarget
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\package-release.ps1 -Target $compilerTarget
```

Ambos pasos usan el destino de la cadena de Rust activa. En Windows x64 es `x86_64-pc-windows-msvc` o `x86_64-pc-windows-gnu`. El ejecutable está en `target/<target>/release/`. El archivo y su fichero `.sha256` se escriben en `target/dist/`. Cada paquete incluye el ejecutable, la licencia, el README y `sample.zz`. Los destinos Unix usan archivos `.tar.gz`.

<a name="publish-a-compiler-release"></a>

### Publicar una versión del compilador

El flujo de publicación compila y prueba binarios nativos x64 y ARM64 para Windows, macOS y Linux. Sube los archivos de cada plataforma, sus sumas SHA-256, ambos instaladores y un archivo `SHA256SUMS` combinado.

Después de registrar y enviar los cambios, publica una etiqueta que coincida con la versión de `Cargo.toml`.

```sh
git tag lmdred-v1.2.0
git push origin lmdred-v1.2.0
```

La etiqueta inicia el flujo, que crea la versión después de que todas las compilaciones pasen. Publicar manualmente una versión de GitHub con la misma etiqueta también inicia el flujo y adjunta los archivos a esa versión. **Run workflow** genera artefactos descargables de Actions para la rama elegida.

Antes de enviar la etiqueta de una versión posterior, actualiza la versión del compilador en `Cargo.toml` y `Cargo.lock`, la versión predeterminada de ambos instaladores y los comandos de instalación del README. Las etiquetas del compilador empiezan por `lmdred-v`. La extensión de VS Code usa etiquetas `drizzle-` y su propia versión en `drizzle/package.json`.

<a name="roadmap"></a>

## Hoja de ruta

- Crear un diagrama de máquina de estados de la gramática Pest de Lamda RED.
- Crear un servicio web con una aplicación React que solicite imágenes PNG a un servidor Rust.

<a name="license"></a>

## Licencia

Este proyecto se distribuye bajo la licencia [GNU GPLv3](../../LICENSE).
