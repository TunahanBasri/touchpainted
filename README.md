# touchpa(inte)d — Windows Precision Touchpad Gesture Launcher

<p align="center">
  <img src="assets/banner.svg" alt="touchpa(inte)d — Windows 10/11 Precision Touchpad Gesture Launcher Banner" width="100%"/>
</p>

<p align="center">
  <a href="https://github.com/TunahanBasri/touchpainted/actions"><img src="https://img.shields.io/github/actions/workflow/status/TunahanBasri/touchpainted/release.yml?branch=main&style=flat-square&label=Build&color=46FFC8&labelColor=15181F" alt="Build Status"/></a>
  <a href="https://github.com/TunahanBasri/touchpainted/releases/latest"><img src="https://img.shields.io/github/v/release/TunahanBasri/touchpainted?style=flat-square&label=Release&color=00D4FF&labelColor=15181F" alt="Latest Release"/></a>
  <img src="https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-46FFC8?style=flat-square&labelColor=15181F" alt="Platform Windows 10/11"/>
  <img src="https://img.shields.io/badge/Rust-Zero%20Crates-00D4FF?style=flat-square&labelColor=15181F" alt="Zero Dependencies Rust"/>
  <img src="https://img.shields.io/badge/Binary-~230%20KB-46FFC8?style=flat-square&labelColor=15181F" alt="Binary Size 230 KB"/>
  <img src="https://img.shields.io/badge/Touchpad-HID%200x000D%3A0x0005-00D4FF?style=flat-square&labelColor=15181F" alt="Raw HID Digitizer"/>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Non--Commercial-FFBE3C?style=flat-square&labelColor=15181F" alt="Non-Commercial Personal License"/></a>
</p>

<p align="center">
  <a href="#-türkçe-genel-bakış"><strong>🇹🇷 Türkçe</strong></a> ·
  <a href="#-english-overview"><strong>🇬🇧 English</strong></a> ·
  <a href="https://github.com/TunahanBasri/touchpainted/releases/latest"><strong>📥 İndir / Download (.exe)</strong></a> ·
  <a href="#️-yapılandırma--configuration-configjson"><strong>⚙️ Yapılandırma / Config</strong></a> ·
  <a href="#-mimari--architecture"><strong>📐 Mimari</strong></a> ·
  <a href="#-sıkça-sorulan-sorular--faq"><strong>❓ SSS / FAQ</strong></a>
</p>

---

<p align="center">
  <img src="assets/demo.gif" alt="touchpa(inte)d demo — double-tap laptop touchpad and draw letter C to launch apps on Windows" width="800"/>
</p>

---

## 🇹🇷 Türkçe Genel Bakış

**touchpa(inte)d** (`touchpainted`), dizüstü bilgisayarınızın **Windows 10 / 11 Precision Touchpad** yüzeyinde **1 veya 2 parmağınızla sıfır baskıyla harf çizerek** (`C`, `V`, `M`, `S`, `W` veya kendi öğrettiğiniz özel şekiller) uygulamaları, klasörleri ve web bağlantılarını saliseler içinde açmanızı sağlayan **sıfır harici bağımlılıklı (Zero-Dependency)** yerel bir Windows hızlı başlatıcısıdır.

### ✨ Öne Çıkan Özellikler
- **⚡ ~230 KB Tek Dosya & Sıfır Boşta CPU Tüketimi:**
  - Tamamen **Rust** ile doğrudan Win32 (`user32.dll`, `gdi32.dll`, `shell32.dll`, `kernel32.dll`, `ole32.dll`) ve Windows HID (`hid.dll`) FFI çağrıları kullanılarak geliştirilmiştir.
  - Harici crate/paket veya ek çalışma zamanı (runtime) gerektirmez. Olay güdümlü (`GetMessageW`) çalışır; touchpad'e dokunulmadığında tamamen uyur.
- **🖐️ Doğrudan Precision Touchpad (`0x000D:0x0005`) Ham HID Okuyucu:**
  - Fiziksel tıklamaya/bastırmaya gerek kalmadan (**Sıfır Baskı**) 1 veya 2 parmağınızı yüzeyde kaydırarak çizim yapabilirsiniz.
  - **Delta-Centroid (Ortak Merkez) Motoru:** İki parmak yüzeye farklı milisaniyelerde değdiğinde parmaklar arasına düz sıçrama çizgisi çekmez (`0 -> 1 -> 2 -> 1` geçişlerini sönümler).
- **📐 Dinamik Otomatik Sığdırma (`Auto-Fit`) Tuvali:**
  - Parmağınızı touchpad'in neresine koyarsanız koyun çizim merkezden başlar; `C` gibi yukarı kavisli harfler tuval kenarlarına çarpıp düzleşmez, anlık olarak merkeze ölçeklenir.
- **🧠 Hibrit Harf Tanıma (`Geometrik Topoloji` + `$1 Unistroke`) & Canlı Harf Öğretme (`T`):**
  - El yazısı farklarına dayanıklı geometrik kural motoru ile 64-noktalı oran koruyucu `$1 Unistroke` algoritmasını birleştirir.
  - **`[🎯 Harf Öğret (T)]`** modunda alttaki harf butonlarına tıklayarak veya klavyeden `A–Z` seçerek kendi çizim stilinizi `config.json` içine kaydedebilirsiniz.
- **🛡️ Giriş İzolasyonu & Tek Kopya Kilidi:**
  - HUD açıkken arka plandaki sayfalarda istenmeyen kaydırma (`WM_MOUSEWHEEL`) engellenir ve harici fare imlecini oynatmak yanlış çizim oluşturmaz.

### 🎮 Hızlı Kullanım (Türkçe)
1. **İndir ve Çalıştır:** [Releases](https://github.com/TunahanBasri/touchpainted/releases/latest) sayfasından `touchpainted.exe` dosyasını indirip çalıştırın (sağ alt Sistem Tepsisine yerleşir).
2. **Çizim Ekranını Aç:** Touchpad'e **2 parmakla çift dokunun ("Tık-Tık")**, klavyeden **`Ctrl + Space`** tuşlarına basın veya tepsi simgesine sol tıklayın.
3. **Sıfır Baskıyla Çiz:** Touchpad üzerinde 1 veya 2 parmağınızı kaydırarak harfi çizin (`C`, `V`, `M`, `S`, `W`). Parmağınızı kaldırdığınız anda hedef uygulama açılır.
4. **Kendi Çizimini Öğret:** HUD açıkken **`T`** tuşuna basın, alttaki kutucuklardan harfi seçin ve touchpad'de 1 kez çizerek kaydedin.

---

## 🇬🇧 English Overview

**touchpa(inte)d** (`touchpainted`) is an ultra-lightweight, **zero-dependency native gesture launcher for Windows 10 & Windows 11 laptops**. Double-tap your **Windows Precision Touchpad** with two fingers to summon a sleek dark HUD canvas, glide **1 or 2 fingers with zero click pressure** to draw a letter (`C`, `V`, `M`, `S`, `W`, or any custom A–Z glyph you train), and launch applications, folders, system URIs, or websites instantaneously.

### 🚀 Key Highlights
- **Zero External Crates (`~230 KB` Standalone `.exe`):** Pure Rust talking directly to Win32 GDI/User32 and Windows Raw HID (`hid.dll`). No runtime dependencies, offline buildable, and **0.00% idle CPU** via event-driven `GetMessageW` sleeping.
- **Direct Raw HID Digitizer (`0x000D:0x0005`) Engine:** Auto-discovers touchpad contact link collections and `LogicalMin..LogicalMax` X/Y resolution at runtime — hardware-agnostic across modern Windows 10/11 laptops.
- **Delta-Centroid Multi-Finger Smoothing:** Eliminates straight-line artifacts when placing or lifting a second finger (`0 -> 1 -> 2 -> 1` contact transitions).
- **Dynamic Auto-Fit Projection:** Unclamped virtual coordinates dynamically center and scale onto the HUD canvas in real time so upward arcs (such as `'C'`) never clip against window borders.
- **Hybrid Gesture Recognizer + Live Calibration (`[T]`):** Combines invariant geometric topology rules with a 64-point uniform-scaled `$1 Unistroke` matcher, plus 1-stroke custom handwriting training per letter (`A–Z`).

### ⚡ Comparison

| Feature | Traditional Script / Runtime Launchers | **touchpa(inte)d** |
| :--- | :--- | :--- |
| **Binary / Disk Footprint** | 50 MB – 180 MB | **~230 KB (Single `.exe`)** |
| **External Dependencies** | Heavy runtimes / dozens of crates | **0 Crates (Pure Win32 + HID FFI)** |
| **2-Finger Touchpad Drawing** | Blocked by OS scroll (`WM_MOUSEWHEEL`) | **Native Raw HID (`0x000D:0x0005`)** |
| **Pressing / Clicking Required?** | Usually requires click-and-drag | **Zero-Pressure Glide (1 or 2 Fingers)** |
| **Idle CPU Usage** | Background polling timers | **0% Idle (Event-Driven)** |

---

## ⚙️ Yapılandırma / Configuration (`config.json`)

Uygulama ilk çalışmada yanında [`config.json`](config.json) dosyasını otomatik oluşturur (Sistem Tepsisi sağ tık menüsünden de doğrudan açabilirsiniz). Her harfe `.exe` programı, klasör yolu, sistem URI komutu (`ms-settings:`) veya `https://...` web adresi bağlayabilirsiniz:

```json
{
  "idle_timeout_ms": 4000,
  "gestures": [
    {
      "letter": "C",
      "name": "Hesap Makinesi",
      "app": "calc.exe",
      "param": "",
      "custom_points": []
    },
    {
      "letter": "V",
      "name": "Dosya Gezgini",
      "app": "explorer.exe",
      "param": "",
      "custom_points": []
    },
    {
      "letter": "M",
      "name": "Çizim Tuvali",
      "app": "mspaint.exe",
      "param": "",
      "custom_points": []
    },
    {
      "letter": "S",
      "name": "Ayarlar",
      "app": "ms-settings:",
      "param": "",
      "custom_points": []
    },
    {
      "letter": "W",
      "name": "Not Defteri",
      "app": "notepad.exe",
      "param": "",
      "custom_points": []
    }
  ]
}
```

---

## 🛠️ Derleme / Build from Source

- **OS:** Windows 10 / Windows 11 (x64)
- **Toolchain:** Rust (`rustc 1.85+`, MSVC) — *0 external crates, builds completely offline.*

```powershell
git clone https://github.com/TunahanBasri/touchpainted.git
cd touchpainted
cargo build --release

# Run:
.\target\release\touchpainted.exe
```

*(Tanılama logları için / For diagnostic logging, pass `--log` to write `touchpad_log.txt`.)*

---

## 📐 Mimari / Architecture

| Katman / Layer | Teknoloji / API | Görev / Role |
| :--- | :--- | :--- |
| **Donanım HID Katmanı** | `RegisterRawInputDevices` (`0x000D:0x0005`) + `hid.dll` (`HidP_GetValueCaps`, `HidP_GetUsages`, `HidP_GetUsageValue`) | Touchpad yüzeyindeki 5 parmağa kadar `Tip Switch` ve `X/Y` koordinatlarını çalışma zamanında otomatik keşfeder ve okur. |
| **Giriş İzolasyon Katmanı** | `SetWindowsHookExW(WH_MOUSE_LL)` | 2-parmak çift tık algılamasını destekler ve HUD açıkken arka plan kaydırmalarını (`WM_MOUSEWHEEL`) bloke eder. |
| **Hareket & Ölçekleme Katmanı** | `Delta-Centroid` + `Auto-Fit İzdüşüm` | Parmak değişim sıçramalarını filtreler ve çizimi tuval sınırlarına çarptırmadan merkeze ölçekler. |
| **Tanıma Motoru** | `compute_geometric_scores` + `$1 Unistroke` (`64-point Uniform Resample`) | Hem topolojik geometri kuralları hem de açısal şablon benzerliğiyle `%` eşleşme skoru üretir. |
| **Arayüz & Sistem Tepsisi** | Win32 GDI Double-Buffered `BitBlt` + `Shell_NotifyIconW` | Titremesiz (flicker-free) çizim tuvali, tıklanabilir kalibrasyon barı ve sistem tepsisi menüsü sunar. |

---

## ❓ Sıkça Sorulan Sorular / FAQ

<details>
<summary><strong>Hangi dizüstü bilgisayarlarda çalışır? / Which laptops are supported?</strong></summary>
<br/>
Windows 10 veya Windows 11 kullanan ve <em>Windows Precision Touchpad</em> standardını destekleyen tüm modern dizüstü bilgisayarlarda donanım çözünürlüğünü çalışma zamanında otomatik algılayarak çalışır. Ayrıca her zaman <code>Ctrl + Space</code> kısayolu ve sol tıkla fare çizimi yedeği mevcuttur.
</details>

<details>
<summary><strong>Arka planda pil veya işlemci harcar mı? / Does it drain battery in the background?</strong></summary>
<br/>
Hayır. Sürekli dönen bir tarama döngüsü (polling loop) yoktur; uygulama Windows mesaj kuyruğunda (<code>GetMessageW</code>) uyur ve yalnızca touchpad etkileşimi olduğunda uyanır.
</details>

<details>
<summary><strong>Bir web sitesini nasıl bağlarım? / How do I launch a website URL?</strong></summary>
<br/>
<code>config.json</code> içindeki <code>"app"</code> alanına doğrudan <code>"https://..."</code> adresini yazmanız yeterlidir; Windows varsayılan tarayıcınızda otomatik olarak açacaktır.
</details>

---

## 📄 Lisans / License (Ticari Kullanıma Kapalı / Non-Commercial)

Telif Hakkı / Copyright © 2026 **TunahanBasri**. Tüm ticari hakları saklıdır / All commercial rights reserved.

Bu proje **[Ticari Olmayan Kişisel Kullanım Lisansı](LICENSE)** kapsamındadır:
- ✅ **Kişisel Kullanım Serbest (Personal Use Permitted):** Bireysel ve eğitim amaçlı ücretsiz kullanabilir, inceleyebilir ve kendi bilgisayarınız için derleyebilirsiniz.
- ❌ **Ticari Kullanım ve Satış Yasak (No Commercial Use):** Yazılımın, kaynak kodunun veya türevlerinin **satılması, ücretli dağıtılması veya ticari ürünlerde kullanılması** kesinlikle yasaktır.
