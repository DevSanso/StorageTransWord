# StorageTransWord

# ๐๋จ์ด ๋ฒ์ญ ํ๋ก๊ทธ๋จ

---

- ํด๋น ํ๋ก์ ํธ๋, ํํ๊ณ  api๋ฅผ ๋จ์ด๋ฅผ ๋ฒ์ญํ๋ ํ๋ก๊ทธ๋จ ์๋๋ค.
- http ํด๋ผ์ด์ธํธ ๋ผ์ด๋ธ๋ฌ๋ฆฌ๋ฅผ ์ด์ฉํ์ฌ ํํ๊ณ ์ restapi์ ์ ์ํ์ฌ ๋จ์ด ๋ฐ์ดํฐ๋ฅผ ์ ์ก
- ๋ฒ์ญ๋ ๋จ์ด๋ฅผ db์ ์ ์ฅํ์ฌ ๋ณด๊ดํ๋ ํ๋ก์ธ์ค์๋๋ค.
- ์ฝ์ ์ถ๋ ฅ ํ๋ฉด ์ฒ๋ฆฌ์ฉ ๊ฐ์ฒด๋ฅผ ์์ฑํ ์คํ์ ์ ์ฅํ์ฌ ๋ค๋ก๊ฐ๊ธฐ ๊ธฐ๋ฅ์ ๊ตฌํํ์์ต๋๋ค.

![window_stack.gif](readme/window_stack.gif)

# Requirements

---

- **rust lang**
- **sqlite**
- **openssh dev**
- **ํํ๊ณ  api ์ค์  ํ์ผ(**[https://developers.naver.com/docs/papago/papago-nmt-overview.md](https://developers.naver.com/docs/papago/papago-nmt-overview.md))
    
    ```json
    //driver_config.json
    {
    	"client_id":"์ ํ๋ฆฌ์ผ์ด์ ๋ฑ๋ก ์ ๋ฐ๊ธ๋ฐ์ ํด๋ผ์ด์ธํธ ์์ด๋ ๊ฐ",
    	"client_secret":"์ ํ๋ฆฌ์ผ์ด์ ๋ฑ๋ก ์ ๋ฐ๊ธ๋ฐ์ ํด๋ผ์ด์ธํธ ์ํฌ๋ฆฟ ๊ฐ"
    }
    ```
    

# Compile

---

```bash
cargo run
```
