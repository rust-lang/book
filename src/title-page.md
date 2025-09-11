# 러스트 프로그래밍 언어

_Steve Klabnik, Carol Nichols, Chris Krycho 지음  
Rust 커뮤니티 기여_

이 문서는 Rust 1.85.0 (2025-02-17 릴리스) 이후 버전을 사용한다고 가정합니다.  
또한 모든 프로젝트의 Cargo.toml 파일에 `edition = "2024"`을 설정하여 Rust 2024  
에디션 규칙을 따르는 것을 전제로 합니다.  

Rust를 설치하거나 업데이트하는 방법은 [1장의 “설치” 절][install]<!-- ignore -->을  
참고해 주세요.  

---

HTML 형식의 문서는 온라인에서 보실 수 있습니다:  
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)  

또한 `rustup`으로 Rust를 설치하셨다면, 오프라인에서도 보실 수 있습니다.  
`rustup doc --book` 명령을 실행하면 책을 바로 열 수 있습니다.  

---

여러 커뮤니티에서 제작한 [번역본][translations]도 제공되고 있습니다.  

이 책은 [No Starch Press][nsprust]에서 종이책과 전자책으로도 만나보실 수 있습니다.  

[install]: ch01-01-installation.html  
[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition  
[translations]: appendix-06-translation.html  

---

> **🚨 더 상호작용적인 학습 경험을 원하시나요?**  
> 퀴즈, 하이라이트, 시각화 등 다양한 기능이 포함된 Rust Book의  
> 다른 버전도 확인해 보세요:  
> <https://rust-book.cs.brown.edu>

# 저작권 및 라이선스 고지

이 책은 *The Rust Programming Language (2024 Edition)* 의 한국어 번역본입니다.  
원저작물은 Rust 커뮤니티와 기여자들에 의해 작성되었으며, MIT 라이선스와  
Apache License (Version 2.0)의 **이중 라이선스(dual license)** 로 배포됩니다.  

본 번역본 역시 동일한 라이선스 조건을 따릅니다. 따라서 누구든지 본 번역본을  
복제, 배포, 수정할 수 있으며, 그 과정에서 다음 조건을 반드시 준수해야 합니다.  

- 원저작권 고지와 라이선스 문구를 삭제하지 말 것  
- 원본과 동일하게 MIT 혹은 Apache 2.0 라이선스 조건을 적용할 것  
- 번역, 수정, 재배포 과정에서 발생한 변경 사항을 명시할 것  

원문은 아래에서 확인하실 수 있습니다.  
<https://github.com/rust-lang/book>  

---

## MIT License

Copyright (c) 2014-2025 The Rust Project Developers

Permission is hereby granted, free of charge, to any person obtaining a copy of this
software and associated documentation files (the "Software"), to deal in the Software
without restriction, including without limitation the rights to use, copy, modify,
merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to the following
conditions:

(이하 생략 — 전체 문구는 <https://opensource.org/licenses/MIT> 참고)  

---

## Apache License, Version 2.0

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

<http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software distributed under
the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied. See the License for the specific language governing
permissions and limitations under the License.
