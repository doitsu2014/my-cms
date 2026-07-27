// =============================================================
// About page content. The structural shape (pillar titles,
// section labels, practice numbering, contact link labels) is
// pinned by `design/new-design/about.html` — keep it stable.
// The narrative copy, timeline entries, statement, and contact
// details below are editable.
//
// Set `verified: false` to hide the page in a locale and show
// the "Coming soon" placeholder instead.
// =============================================================

export interface AboutLink {
  label: string;
  url: string;
  handle?: string;
}

export interface AboutFact {
  label: string;
  value: string;
}

export interface AboutPillar {
  label: string;
  title: string;
  titleAccent?: string;
  subtitle: string;
  paragraphs: string[];
}

export interface AboutSectionHeader {
  eyebrow: string;
  title: string;
  lead: string;
}

export interface AboutPractice {
  title: string;
  body: string;
}

export interface AboutTimelineEntry {
  date: string;
  title: string;
  eyebrow: string;
  body: string;
}

export interface AboutTimeline {
  eyebrow: string;
  title: string;
  lead: string;
  entries: AboutTimelineEntry[];
}

export interface AboutContent {
  verified: boolean;
  hero: {
    eyebrow: string;
    title: string;
    subtitle: string;
    body: string;
    facts: [AboutFact, AboutFact, AboutFact];
  };
  pillars: AboutPillar[];
  pillarsSection: AboutSectionHeader;
  domains: AboutPillar[];
  domainsSection: AboutSectionHeader;
  statement: { quote: string; cite: string };
  timeline: AboutTimeline;
  practices: AboutPractice[];
  contact: {
    eyebrow: string;
    title: string;
    body: string;
    email?: string;
    links: AboutLink[];
    recentPost?: { label: string; date: string; url: string };
  };
}

const enContent = (): AboutContent => ({
  verified: true,
  hero: {
    eyebrow: 'Senior software engineer',
    title: 'Seven years of distributed systems.',
    subtitle:
      "I build cloud-native platforms for financial workflows — the kind where reliability isn't a feature, it's the cost of admission. Currently leading architecture for IRIS at Dragon Capital.",
    body:
      "I work mostly in C# and Rust, deploy on Azure and Kubernetes, and care about the gap between what's shipped and what actually runs well in production. Based in Ho Chi Minh City.",
    facts: [
      { label: 'Practice', value: 'Cloud-native · Distributed systems' },
      { label: 'Experience', value: '7+ years · 2017 — present' },
      { label: 'Current role', value: 'Senior Software Engineer · Dragon Capital' },
    ],
  },
  pillars: [
    {
      label: '01 · Engineering',
      title: 'Backend',
      titleAccent: 'systems',
      subtitle: 'Cloud-native · C#, Rust, ASP.NET Core · 2017 — present',
      paragraphs: [
        'I build backend systems that need to keep running when something fails — distributed services, financial data pipelines, and order-management workflows on Azure AKS and Kubernetes.',
        'Languages and tools I reach for: C#, Rust, TypeScript, Kafka, PostgreSQL, Redis, ElasticSearch, Terraform. The shape of the system matters more than the framework.',
      ],
    },
    {
      label: '02 · Operations',
      title: 'Reliability',
      titleAccent: '& ops',
      subtitle: 'Production reliability · CI/CD · on-call',
      paragraphs: [
        "I treat reliability as a feature, not a footnote. At Dragon Capital I modernized the IRIS CI/CD pipeline, improved database architecture, and rolled out observability that catches problems before traders do.",
        "Deploys should be boring. If a deployment is exciting, the system is telling you something.",
      ],
    },
    {
      label: '03 · Leadership',
      title: 'Mentor',
      titleAccent: '& lead',
      subtitle: 'Tech lead · 12–15 engineers · AI-assisted workflow',
      paragraphs: [
        "I've led teams of 12–15 engineers at Reso, owned knowledge transfer and internalization of mission-critical systems at Dragon Capital, and mentored engineers through refactors and onboarding.",
        'Recently I rolled out GitHub Copilot Enterprise across the IRIS team — onboarding, refactoring, and code comprehension got noticeably faster.',
      ],
    },
  ],
  pillarsSection: {
    eyebrow: 'Career pillars',
    title: 'Three pillars of seven years.',
    lead: 'Most lessons came from running systems in production, handling incidents, and working with engineers I learned something from — not from meeting rooms.',
  },
  domains: [
    {
      label: '01 · Retail & operations',
      title: 'Wifi Market',
      titleAccent: 'Event · POS · CMS · CRM',
      subtitle: 'WiSky · Reso · ExE · 2017 — 2019',
      paragraphs: [
        'At WiSky I built WiFi marketing platforms, attendance systems, and an internal web builder framework that accelerated site delivery. At ExE I integrated Azure Face Recognition into attendance tracking and built logistics support utilities for warehouse operations.',
        'At Reso I led the development of CRM and POS systems for retail and F&B businesses — the operational backbone for hundreds of merchants — while managing a team of 12–15 engineers.',
      ],
    },
    {
      label: '02 · B2B payment gateway',
      title: 'B2B Payment',
      titleAccent: 'Gateway',
      subtitle: 'Easy IT · eTreem · 2019 — 2024',
      paragraphs: [
        "At Easy IT Solution on the Amex OneAP project I shipped the CPG Gateway handling high-volume financial transaction requests, the Identity Security Service for auth, and optimized background workers for payment processing — enterprise-grade B2B payment infrastructure under Amex's engineering standards.",
        'At eTreem I built the Payrix Gateway Service integrating enterprise payment infrastructure, designed a centralized Identity Server for the microservice ecosystem, and architected the platform on Azure Kubernetes Service with distributed observability via Elastic Stack and Jaeger.',
      ],
    },
    {
      label: '03 · Financial · fund management',
      title: 'Financial',
      titleAccent: '& fund management',
      subtitle: 'Dragon Capital · 2024 — present',
      paragraphs: [
        'At Dragon Capital I own architecture and operational stability of IRIS — the core investment platform covering portfolio management, OMS, Portfolio Rebalancing, ISBD data pipelines, and Algo X trading integration. Real money, real reliability constraints.',
        'The IRIS platform supports research, portfolio management, front-office trading, middle office, reporting, and data distribution — the full lifecycle of fund operations.',
      ],
    },
  ],
  domainsSection: {
    eyebrow: 'Career domains',
    title: 'Three industries, seven years.',
    lead: 'From WiFi marketing tools to fund management platforms handling real money — same engineering discipline, very different problem shapes.',
  },
  timeline: {
    eyebrow: 'Career timeline',
    title: 'Seven years, five chapters.',
    lead: 'Each chapter is a lesson — different teams, different problems, one through-line: distributed systems that actually work in production.',
    entries: [
      {
        date: '2017',
        title: 'First production code at WiSky',
        eyebrow: 'WiSky · Junior Software Developer',
        body: 'Started as a junior building WiFi marketing platforms, attendance systems, and an internal web builder framework. Learned the value of caching, query optimization, and shipping.',
      },
      {
        date: '2019 — 2022',
        title: 'High-volume financial microservices with Amex OneAP',
        eyebrow: 'Easy IT Solution · Amex OneAP · Middle Software Engineer',
        body: 'Full-time on the Amex OneAP team building enterprise financial microservices. Shipped the CPG Gateway handling high-volume transaction requests, the Identity Security Service for auth, and optimized background workers for payment processing.',
      },
      {
        date: '2022 — 2024',
        title: 'Payment gateway and Azure-native platform at eTreem',
        eyebrow: 'eTreem · Senior Software Engineer (contractor) · Azure AKS · Payrix',
        body: 'Built the Payrix Gateway Service integrating enterprise payment infrastructure, designed a centralized Identity Server for the microservice ecosystem, and architected the platform on Azure Kubernetes Service with distributed observability via Elastic Stack and Jaeger.',
      },
      {
        date: 'Now',
        title: 'Leading IRIS architecture at Dragon Capital',
        eyebrow: 'Dragon Capital · Senior Software Engineer · IRIS · OMS · Algo X · ISBD',
        body: 'Own architecture and operational stability of IRIS — the core investment platform — plus OMS, Portfolio Rebalancing, ISBD data pipelines, and Algo X trading integration. Real money, real reliability constraints.',
      },
      {
        date: 'Recent',
        title: 'AI-assisted engineering workflow',
        eyebrow: 'GitHub Copilot Enterprise · onboarding · refactoring',
        body: 'Introduced an AI-assisted development workflow across the IRIS team. Accelerated onboarding, code comprehension, and refactoring — and changed how I think about leverage in engineering work.',
      },
    ],
  },
  statement: {
    quote:
      "Money doesn't lie. Production either works or it doesn't — everything else is a story we tell ourselves.",
    cite: 'Personal note, after seven years operating financial systems in production.',
  },
  practices: [
    { title: '01 · Read the system before changing it', body: "Understand before editing. The fastest way to break a distributed system is to change it before you understand why it's shaped that way." },
    { title: '02 · Make deploys boring', body: 'If a deployment is exciting, something is wrong. Boring deploys are the result of good CI/CD, automation, and runbooks — not luck.' },
    { title: '03 · Migrate vendor code, eventually', body: 'Vendor code is a starting point, not a permanent dependency. Plan the internalization early, even if it happens late.' },
    { title: '04 · Use AI as a pair, not a crutch', body: "Engineers who use AI will replace those who don't. But the value comes from your judgment, not the autocomplete." },
  ],
  contact: {
    eyebrow: 'Get in touch',
    title: 'Open to conversations.',
    body: 'I respond to thoughtful messages about cloud-native platforms, FinTech architecture, distributed systems, and engineering team modernization. Replies usually within a couple of days.',
    email: 'thd1152015@gmail.com',
    links: [
      { label: 'GitHub', url: 'https://github.com/doitsu2014', handle: '@doitsu2014' },
      { label: 'LinkedIn', url: 'https://www.linkedin.com/in/duc-tran-huu-167b1612a/', handle: 'duc-tran-huu' },
      { label: 'RSS / Atom', url: '/feed.xml', handle: '/feed.xml' },
    ],
    recentPost: {
      label: 'Recent post',
      date: 'No posts yet',
      url: '/en/posts',
    },
  },
});

const viContent = (): AboutContent => ({
  verified: true,
  hero: {
    eyebrow: 'Kỹ sư phần mềm cao cấp',
    title: 'Bảy năm với distributed systems.',
    subtitle:
      'Mình xây cloud-native platforms cho workflow tài chính — loại hệ thống mà reliability không phải feature, mà là điều kiện tiên quyết. Hiện đang phụ trách kiến trúc IRIS tại Dragon Capital.',
    body:
      'Làm chủ yếu với C# và Rust, deploy trên Azure và Kubernetes, và quan tâm đến khoảng cách giữa thứ được ship và thứ thực sự chạy ổn trên production. Mình ở TP. Hồ Chí Minh.',
    facts: [
      { label: 'Nghề', value: 'Cloud-native · Distributed systems' },
      { label: 'Kinh nghiệm', value: '7+ năm · 2017 — nay' },
      { label: 'Vai trò hiện tại', value: 'Senior Software Engineer · Dragon Capital' },
    ],
  },
  pillars: [
    {
      label: '01 · Kỹ sư',
      title: 'Backend',
      titleAccent: 'systems',
      subtitle: 'Cloud-native · C#, Rust, ASP.NET Core · 2017 — nay',
      paragraphs: [
        'Mình xây backend system cần chạy ổn khi có thứ gì đó fail — distributed service, financial data pipeline, và order-management workflow trên Azure AKS và Kubernetes.',
        'Ngôn ngữ và công cụ mình dùng: C#, Rust, TypeScript, Kafka, PostgreSQL, Redis, ElasticSearch, Terraform. Shape của hệ thống quan trọng hơn framework.',
      ],
    },
    {
      label: '02 · Vận hành',
      title: 'Reliability',
      titleAccent: '& ops',
      subtitle: 'Production reliability · CI/CD · on-call',
      paragraphs: [
        'Mình coi reliability là feature, không phải footnote. Ở Dragon Capital, mình modernize CI/CD pipeline của IRIS, cải thiện database architecture, và triển khai observability để bắt vấn đề trước khi trader kịp nhận ra.',
        'Deploy nên nhàm chán. Nếu một deployment gây hào hứng, hệ thống đang nói với bạn điều gì đó.',
      ],
    },
    {
      label: '03 · Dẫn dắt',
      title: 'Mentor',
      titleAccent: '& lead',
      subtitle: 'Tech lead · 12–15 engineers · AI-assisted workflow',
      paragraphs: [
        'Mình từng lead team 12–15 người ở Reso, chịu trách nhiệm knowledge transfer và internalization cho các hệ thống mission-critical tại Dragon Capital, và mentor engineer qua các đợt refactor cùng onboarding.',
        'Gần đây mình rollout GitHub Copilot Enterprise cho cả team IRIS — onboarding, refactoring, và code comprehension nhanh hơn rõ rệt.',
      ],
    },
  ],
  pillarsSection: {
    eyebrow: 'Hành trình nghề · Career pillars',
    title: 'Ba trụ cột của bảy năm qua.',
    lead: 'Phần lớn bài học đến từ việc vận hành hệ thống production, xử lý sự cố, và làm việc với những engineer mà mình học được điều gì đó — không phải từ phòng họp.',
  },
  domains: [
    {
      label: '01 · Bán lẻ & vận hành',
      title: 'Wifi Market',
      titleAccent: 'Event · POS · CMS · CRM',
      subtitle: 'WiSky · Reso · ExE · 2017 — 2019',
      paragraphs: [
        'Ở WiSky mình build WiFi marketing platform, attendance system, và một internal web builder framework tăng tốc độ giao hàng site. Ở ExE tích hợp Azure Face Recognition vào attendance tracking và xây logistics support utility cho warehouse operation.',
        'Ở Reso mình lead phát triển CRM và POS cho doanh nghiệp retail và F&B — operational backbone cho hàng trăm merchant — đồng thời quản lý team 12–15 engineer.',
      ],
    },
    {
      label: '02 · B2B payment gateway',
      title: 'B2B Payment',
      titleAccent: 'Gateway',
      subtitle: 'Easy IT · eTreem · 2019 — 2024',
      paragraphs: [
        'Ở Easy IT Solution trong project Amex OneAP mình ship CPG Gateway xử lý financial transaction request cường độ cao, Identity Security Service cho auth, và tối ưu background worker cho payment processing — B2B payment infrastructure cấp enterprise theo chuẩn engineering của Amex.',
        'Ở eTreem mình xây Payrix Gateway Service tích hợp enterprise payment infrastructure, thiết kế Identity Server tập trung cho microservice ecosystem, và kiến trúc platform trên Azure Kubernetes Service với distributed observability qua Elastic Stack và Jaeger.',
      ],
    },
    {
      label: '03 · Tài chính · quản lý quỹ',
      title: 'Financial',
      titleAccent: '& fund management',
      subtitle: 'Dragon Capital · 2024 — nay',
      paragraphs: [
        'Ở Dragon Capital mình chịu trách nhiệm kiến trúc và ổn định vận hành của IRIS — investment platform cốt lõi bao gồm portfolio management, OMS, Portfolio Rebalancing, ISBD data pipeline, và Algo X trading integration. Tiền thật, reliability constraint thật.',
        'IRIS platform hỗ trợ research, portfolio management, front-office trading, middle office, reporting, và data distribution — toàn bộ lifecycle của fund operation.',
      ],
    },
  ],
  domainsSection: {
    eyebrow: 'Lĩnh vực nghề · Career domains',
    title: 'Ba lĩnh vực, bảy năm.',
    lead: 'Từ WiFi marketing tool đến fund management platform xử lý tiền thật — cùng kỷ luật engineering, hình dạng bài toán rất khác nhau.',
  },
  timeline: {
    eyebrow: 'Hành trình nghề · Career timeline',
    title: 'Bảy năm, năm chương.',
    lead: 'Mỗi chương là một bài học — team khác nhau, bài toán khác nhau, một đường thẳng xuyên suốt: distributed system thực sự chạy ổn trên production.',
    entries: [
      {
        date: '2017',
        title: 'Production code đầu tiên tại WiSky',
        eyebrow: 'WiSky · Junior Software Developer',
        body: 'Bắt đầu với vai trò junior, build WiFi marketing platform, attendance system, và một internal web builder framework. Học được giá trị của caching, query optimization, và việc ship thật nhanh.',
      },
      {
        date: '2019 — 2022',
        title: 'Financial microservices cường độ cao với Amex OneAP',
        eyebrow: 'Easy IT Solution · Amex OneAP · Middle Software Engineer',
        body: 'Full-time trong team Amex OneAP xây financial microservices cấp enterprise. Ship CPG Gateway xử lý transaction request cường độ cao, Identity Security Service cho auth, và tối ưu background worker cho payment processing.',
      },
      {
        date: '2022 — 2024',
        title: 'Payment gateway và Azure-native platform tại eTreem',
        eyebrow: 'eTreem · Senior Software Engineer (contractor) · Azure AKS · Payrix',
        body: 'Xây Payrix Gateway Service tích hợp enterprise payment infrastructure, thiết kế Identity Server tập trung cho microservice ecosystem, và kiến trúc platform trên Azure Kubernetes Service với distributed observability qua Elastic Stack và Jaeger.',
      },
      {
        date: 'Hiện tại',
        title: 'Phụ trách kiến trúc IRIS tại Dragon Capital',
        eyebrow: 'Dragon Capital · Senior Software Engineer · IRIS · OMS · Algo X · ISBD',
        body: 'Chịu trách nhiệm kiến trúc và ổn định vận hành của IRIS — investment platform cốt lõi — cùng OMS, Portfolio Rebalancing, ISBD data pipeline, và Algo X trading integration. Tiền thật, reliability constraint thật.',
      },
      {
        date: 'Gần đây',
        title: 'AI-assisted engineering workflow',
        eyebrow: 'GitHub Copilot Enterprise · onboarding · refactoring',
        body: 'Giới thiệu AI-assisted development workflow cho cả team IRIS. Onboarding, code comprehension, và refactoring nhanh hơn — và thay đổi cách mình nghĩ về đòn bẩy trong engineering work.',
      },
    ],
  },
  statement: {
    quote:
      'Tiền không nói dối. Production hoặc chạy hoặc không — mọi thứ khác chỉ là câu chuyện ta tự kể với nhau.',
    cite: 'Ghi chép cá nhân, sau bảy năm vận hành hệ thống tài chính production.',
  },
  practices: [
    { title: '01 · Đọc hệ thống trước khi thay đổi', body: 'Hiểu trước khi sửa. Cách nhanh nhất để break một distributed system là thay đổi nó trước khi hiểu vì sao nó có shape như vậy.' },
    { title: '02 · Làm cho deploy nhàm chán', body: 'Nếu một deployment gây hào hứng, có gì đó đang sai. Deploy nhàm chán là kết quả của CI/CD tốt, automation, và runbook — không phải may mắn.' },
    { title: '03 · Migrate code vendor, sớm hay muộn', body: 'Code vendor là điểm khởi đầu, không phải dependency vĩnh viễn. Plan internalization sớm, dù nó có xảy ra muộn.' },
    { title: '04 · Dùng AI như pair, không phải nạng', body: 'Engineer biết dùng AI sẽ thay thế engineer không biết dùng. Nhưng giá trị đến từ phán đoán của bạn, không phải từ autocomplete.' },
  ],
  contact: {
    eyebrow: 'Giữ liên lạc · Get in touch',
    title: 'Sẵn sàng cho các cuộc trò chuyện.',
    body: 'Mình phản hồi những email có nội dung về cloud-native platforms, FinTech architecture, distributed systems, và engineering team modernization. Thường trong vòng vài ngày.',
    email: 'thd1152015@gmail.com',
    links: [
      { label: 'GitHub', url: 'https://github.com/doitsu2014', handle: '@doitsu2014' },
      { label: 'LinkedIn', url: 'https://www.linkedin.com/in/duc-tran-huu-167b1612a/', handle: 'duc-tran-huu' },
      { label: 'RSS / Atom', url: '/feed.xml', handle: '/feed.xml' },
    ],
    recentPost: {
      label: 'Bài viết gần nhất',
      date: 'Chưa có bài viết',
      url: '/vi/posts',
    },
  },
});

export const ABOUT_CONFIG: Record<'en' | 'vi', AboutContent> = {
  en: enContent(),
  vi: viContent(),
};

export function getAboutContent(lang: string): AboutContent {
  return ABOUT_CONFIG[lang === 'vi' ? 'vi' : 'en'];
}