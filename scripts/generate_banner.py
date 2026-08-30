import os
import base64
import subprocess
from PIL import Image

def generate_banner():
    # 1. Ensure cropped logo exists
    logo_path = 'assets/sorayunara-transparent.png'
    img = Image.open(logo_path)
    crop_box = (196, 141, 509, 692)
    cropped = img.crop(crop_box)
    cropped.save('assets/logo_cropped.png', 'PNG')

    with open('assets/logo_cropped.png', 'rb') as f:
        logo_b64 = base64.b64encode(f.read()).decode('utf-8')

    html_content = f'''<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<style>
  * {{
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }}
  @import url('https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700;800;900&family=JetBrains+Mono:wght@500;700&display=swap');
  
  body {{
    width: 1280px;
    height: 360px;
    overflow: hidden;
    background: #090d16;
    font-family: 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    color: #ffffff;
    display: flex;
    align-items: center;
    position: relative;
    user-select: none;
  }}

  /* Ambient Glow & Grid Background */
  .bg-grid {{
    position: absolute;
    inset: 0;
    background-size: 36px 36px;
    background-image: 
      linear-gradient(to right, rgba(255, 255, 255, 0.035) 1px, transparent 1px),
      linear-gradient(to bottom, rgba(255, 255, 255, 0.035) 1px, transparent 1px);
    mask-image: radial-gradient(circle at 40% 50%, black 40%, transparent 85%);
  }}

  .glow-1 {{
    position: absolute;
    width: 550px;
    height: 360px;
    left: -80px;
    top: -60px;
    background: radial-gradient(circle, rgba(124, 58, 237, 0.38) 0%, rgba(99, 102, 241, 0.15) 50%, transparent 75%);
    filter: blur(40px);
    pointer-events: none;
  }}

  .glow-2 {{
    position: absolute;
    width: 600px;
    height: 360px;
    right: -100px;
    bottom: -60px;
    background: radial-gradient(circle, rgba(14, 165, 233, 0.28) 0%, rgba(236, 72, 153, 0.18) 50%, transparent 75%);
    filter: blur(45px);
    pointer-events: none;
  }}

  .glow-accent {{
    position: absolute;
    width: 250px;
    height: 250px;
    left: 120px;
    top: 55px;
    background: radial-gradient(circle, rgba(168, 85, 247, 0.45) 0%, transparent 70%);
    filter: blur(35px);
  }}

  /* Main Container */
  .container {{
    position: relative;
    z-index: 10;
    width: 100%;
    height: 100%;
    padding: 0 54px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }}

  /* Left Logo Area */
  .logo-box {{
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    width: 210px;
    height: 210px;
    flex-shrink: 0;
    margin-right: 40px;
  }}

  .logo-backdrop {{
    position: absolute;
    inset: 4px;
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.08), rgba(255, 255, 255, 0.02));
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 28px;
    box-shadow: 
      0 20px 40px -15px rgba(0, 0, 0, 0.7),
      inset 0 1px 1px rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(12px);
  }}

  .logo-img {{
    position: relative;
    z-index: 2;
    height: 156px;
    width: auto;
    filter: drop-shadow(0 10px 22px rgba(124, 58, 237, 0.5));
  }}

  /* Right Content Area */
  .content {{
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }}

  .header-row {{
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 6px;
  }}

  .title {{
    font-size: 50px;
    font-weight: 900;
    letter-spacing: -1.2px;
    background: linear-gradient(135deg, #ffffff 30%, #e2e8f0 70%, #94a3b8 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    text-transform: uppercase;
    line-height: 1;
  }}

  .lang-tag {{
    font-family: 'JetBrains Mono', monospace;
    font-size: 15px;
    font-weight: 700;
    color: #c084fc;
    background: rgba(192, 132, 252, 0.12);
    border: 1px solid rgba(192, 132, 252, 0.35);
    padding: 4px 12px;
    border-radius: 8px;
    letter-spacing: 0.5px;
  }}

  .version-tag {{
    font-family: 'JetBrains Mono', monospace;
    font-size: 13px;
    font-weight: 700;
    color: #38bdf8;
    background: rgba(56, 189, 248, 0.1);
    border: 1px solid rgba(56, 189, 248, 0.3);
    padding: 4px 10px;
    border-radius: 8px;
  }}

  .tagline {{
    font-size: 17.5px;
    font-weight: 600;
    color: #94a3b8;
    margin-bottom: 22px;
    letter-spacing: -0.2px;
  }}

  .tagline span {{
    color: #f8fafc;
  }}

  /* Feature Badges Grid */
  .features-grid {{
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
  }}

  .feat-card {{
    background: rgba(255, 255, 255, 0.035);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    padding: 10px 14px;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }}

  .feat-top {{
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    font-weight: 700;
    color: #f1f5f9;
  }}

  .feat-sub {{
    font-size: 11.5px;
    font-weight: 500;
    color: #64748b;
  }}

  /* Corner Brand Watermark */
  .bottom-bar {{
    position: absolute;
    bottom: 12px;
    right: 54px;
    font-size: 11px;
    font-family: 'JetBrains Mono', monospace;
    font-weight: 500;
    color: #475569;
    letter-spacing: 0.5px;
  }}
</style>
</head>
<body>
  <div class="bg-grid"></div>
  <div class="glow-1"></div>
  <div class="glow-2"></div>
  <div class="glow-accent"></div>

  <div class="container">
    <div class="logo-box">
      <div class="logo-backdrop"></div>
      <img class="logo-img" src="data:image/png;base64,{logo_b64}" alt="Sorayunara Logo" />
    </div>

    <div class="content">
      <div class="header-row">
        <div class="title">SORAYUNARA</div>
        <div class="lang-tag">.sora</div>
        <div class="version-tag">v0.2.2</div>
      </div>

      <div class="tagline">
        The <span>Next-Gen Systems & AI</span> Programming Language
      </div>

      <div class="features-grid">
        <div class="feat-card">
          <div class="feat-top"><span>⚡</span> Zero-Cost Safety</div>
          <div class="feat-sub">Borrow checking & regions</div>
        </div>

        <div class="feat-card">
          <div class="feat-top"><span>🧠</span> HM Inference</div>
          <div class="feat-sub">Type unification & ADT</div>
        </div>

        <div class="feat-card">
          <div class="feat-top"><span>🎭</span> Lock-Free Actors</div>
          <div class="feat-sub">Multi-core concurrency</div>
        </div>

        <div class="feat-card">
          <div class="feat-top"><span>🚀</span> Multi-Backend</div>
          <div class="feat-sub">Native LLVM, WASM & C99</div>
        </div>
      </div>
    </div>
  </div>

  <div class="bottom-bar">
    sorayunara.org • github.com/Sorayunara
  </div>
</body>
</html>
'''

    with open('assets/banner.html', 'w', encoding='utf-8') as f:
        f.write(html_content)
    
    print('banner.html generated.')

    # 2. Render with headless Chrome
    chrome_path = r'C:\Program Files\Google\Chrome\Application\chrome.exe'
    abs_html = os.path.abspath('assets/banner.html')
    output_png = os.path.abspath('assets/banner.png')

    cmd = [
        chrome_path,
        '--headless',
        '--disable-gpu',
        '--force-device-scale-factor=1',
        '--window-size=1280,360',
        '--hide-scrollbars',
        f'--screenshot={output_png}',
        f'file:///{abs_html}'
    ]
    
    print('Running Chrome screenshot...')
    subprocess.run(cmd, check=True)

    # Crop to exact 1280x360 if Chrome added any padding
    rendered = Image.open(output_png)
    if rendered.size != (1280, 360):
        rendered = rendered.crop((0, 0, 1280, 360))
        rendered.save(output_png, 'PNG')

    print(f'Banner rendered successfully to {output_png} (Size: {rendered.size})')

if __name__ == '__main__':
    generate_banner()
