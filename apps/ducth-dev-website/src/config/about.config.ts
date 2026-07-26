// =============================================================
// About page content. Edit the `body`, `subtitle`, `paragraphs`,
// and `email` fields to replace the prototype draft copy with
// your own narrative. The structural labels, pillar titles,
// practice titles, and contact handle names follow the
// `design/new-design/about.html` source.
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
    eyebrow: 'Software engineer',
    title: 'Eleven years writing software.',
    subtitle:
      'TODO: write the lead paragraph that introduces you. Keep it to two or three sentences; this is the hook before the visitor scrolls into the pillars.',
    body:
      'TODO: optional second paragraph. Mention your location, current focus, and what this notebook is for.',
    facts: [
      { label: 'Practice', value: 'Backend · Distributed systems' },
      { label: 'Experience', value: '11 years · 2014 — present' },
      { label: 'Current role', value: 'Senior Software Engineer' },
    ],
  },
  pillars: [
    {
      label: '01 · Engineering',
      title: 'Backend',
      titleAccent: 'systems',
      subtitle: 'Backend engineering · 2014 — present',
      paragraphs: [
        'TODO: describe your backend practice — languages, runtime scale, the kind of problems you solve.',
        'TODO: list the tools you reach for (TypeScript, Go, Python, Postgres, Redis, Kafka, Kubernetes) only if you want to anchor the reader in concrete terms.',
      ],
    },
    {
      label: '02 · Operations',
      title: 'Reliability',
      titleAccent: '& ops',
      subtitle: 'Production reliability · on-call · SRE',
      paragraphs: [
        'TODO: describe your on-call and incident-response experience.',
        'TODO: share the principle that guides your reliability work.',
      ],
    },
    {
      label: '03 · Leadership',
      title: 'Mentor',
      titleAccent: '& lead',
      subtitle: 'Tech lead · mentoring · technical writing',
      paragraphs: [
        'TODO: describe your mentoring and code-review practice.',
        'TODO: mention team size and any concrete numbers you are comfortable sharing.',
      ],
    },
  ],
  pillarsSection: {
    eyebrow: 'Career pillars',
    title: 'Three pillars of eleven years.',
    lead: 'TODO: one or two sentences that frame the pillars. Not every lesson came from a meeting room — most came from the systems I have run, the incidents I have handled, and the people I have worked with.',
  },
  timeline: {
    eyebrow: 'Career timeline',
    title: 'Eleven years, three chapters.',
    lead:
      'Each chapter is a lesson — from the first line of code to financial systems in production.',
    entries: [
      {
        date: '2014',
        title: 'First line of production code',
        eyebrow: 'First line of production code',
        body: 'TODO: write the opening entry — what you shipped, what you learned, why you stayed.',
      },
      {
        date: 'Now',
        title: 'Software Engineer at Dragon Capital',
        eyebrow: 'Saigon · Financial systems · C#, Angular, React',
        body: 'TODO: write the present chapter — employer, scope, and the systems you own.',
      },
      {
        date: 'Recent',
        title: 'AI-assisted development workflow',
        eyebrow: '3–4 AI agents · Tooling around the engineering',
        body: 'TODO: write the recent chapter — the workflow, the tools, and what changed.',
      },
    ],
  },
  statement: {
    quote:
      'TODO: replace with a short, personal statement that captures how you think about engineering. Keep it under 280 characters.',
    cite: 'TODO: attribution (e.g., personal note, after six years operating a production service)',
  },
  practices: [
    { title: '01 · Read before you write', body: 'TODO: one or two sentences on this habit.' },
    { title: '02 · Write slow, delete fast', body: 'TODO: one or two sentences on this habit.' },
    { title: '03 · Measure less, observe more', body: 'TODO: one or two sentences on this habit.' },
    { title: '04 · Let go, selectively', body: 'TODO: one or two sentences on this habit.' },
  ],
  contact: {
    eyebrow: 'Get in touch',
    title: 'If you are hiring a backend engineer.',
    body: 'TODO: write the contact body — your current hiring posture, response-time expectation, and what you are open to discussing. Keep it under 280 characters.',
    email: 'TODO: replace with your real email or remove the field',
    links: [
      { label: 'GitHub', url: 'https://github.com/ductran', handle: '@ductran' },
      { label: 'LinkedIn', url: 'https://www.linkedin.com/in/duc-tran-huu-167b1612a/', handle: 'duc-tran-huu' },
      { label: 'RSS / Atom', url: '/feed.xml', handle: '/feed.xml' },
    ],
    recentPost: {
      label: 'Recent post',
      date: 'TODO: 14 / 03 / 2025',
      url: '/en/posts',
    },
  },
});

const viContent = (): AboutContent => ({
  verified: true,
  hero: {
    eyebrow: 'Kỹ sư phần mềm',
    title: 'Mười một năm viết phần mềm.',
    subtitle:
      'TODO: viết đoạn mở đầu bằng tiếng Việt — hai hoặc ba câu, đây là hook trước khi người đọc cuộn xuống các trụ cột.',
    body:
      'TODO: đoạn thứ hai (tuỳ chọn). Nêu nơi bạn sống, focus hiện tại, và mục đích của blog.',
    facts: [
      { label: 'Nghề', value: 'Backend · Distributed systems' },
      { label: 'Kinh nghiệm', value: '11 năm · 2014 — nay' },
      { label: 'Vai trò hiện tại', value: 'Senior Software Engineer' },
    ],
  },
  pillars: [
    {
      label: '01 · Kỹ sư',
      title: 'Backend',
      titleAccent: 'systems',
      subtitle: 'Backend engineering · 2014 — nay',
      paragraphs: [
        'TODO: mô tả practice backend của bạn — ngôn ngữ, quy mô hệ thống, loại bài toán bạn giải.',
        'TODO: liệt kê công cụ (TypeScript, Go, Python, Postgres, Redis, Kafka, Kubernetes) nếu muốn neo người đọc vào cụ thể.',
      ],
    },
    {
      label: '02 · Vận hành',
      title: 'Reliability',
      titleAccent: '& ops',
      subtitle: 'Production reliability · on-call · SRE',
      paragraphs: [
        'TODO: mô tả kinh nghiệm on-call và incident response.',
        'TODO: chia sẻ nguyên tắc dẫn dắt reliability work của bạn.',
      ],
    },
    {
      label: '03 · Dẫn dắt',
      title: 'Mentor',
      titleAccent: '& lead',
      subtitle: 'Tech lead · mentoring · technical writing',
      paragraphs: [
        'TODO: mô tả practice mentoring và code review.',
        'TODO: nêu quy mô team và các con số cụ thể bạn muốn chia sẻ.',
      ],
    },
  ],
  pillarsSection: {
    eyebrow: 'Hành trình nghề · Career pillars',
    title: 'Ba trụ cột của mười một năm qua.',
    lead: 'Không phải tất cả đều đến từ phòng họp — phần lớn đến từ những hệ thống tôi đã chạy, những sự cố tôi đã xử lý, và những người tôi đã làm việc cùng.',
  },
  timeline: {
    eyebrow: 'Hành trình nghề · Career timeline',
    title: 'Mười một năm, ba chương.',
    lead:
      'Mỗi chương là một bài học — từ dòng code đầu tiên đến hệ thống tài chính production. Each chapter is a lesson — from the first line of code to financial systems in production.',
    entries: [
      {
        date: '2014',
        title: 'Bắt đầu viết production code',
        eyebrow: 'First line of production code',
        body: 'TODO: viết entry mở đầu — bạn đã ship gì, học được gì, vì sao bạn ở lại.',
      },
      {
        date: 'Now',
        title: 'Software Engineer tại Dragon Capital',
        eyebrow: 'Sài Gòn · Financial systems · C#, Angular, React',
        body: 'TODO: viết entry hiện tại — nơi làm việc, phạm vi, và hệ thống bạn sở hữu.',
      },
      {
        date: 'Recent',
        title: 'AI-assisted development workflow',
        eyebrow: '3–4 AI agents · Tooling around the engineering',
        body: 'TODO: viết entry gần đây — workflow, công cụ, và điều gì đã thay đổi.',
      },
    ],
  },
  statement: {
    quote:
      'TODO: thay bằng một câu ngắn, mang tính cá nhân, nắm bắt cách bạn nghĩ về engineering. Dưới 280 ký tự.',
    cite: 'TODO: nguồn trích dẫn (vd: ghi chép cá nhân, sau 6 năm vận hành một dịch vụ production)',
  },
  practices: [
    { title: '01 · Đọc trước khi viết', body: 'TODO: một hoặc hai câu cho thói quen này.' },
    { title: '02 · Viết chậm, xoá nhanh', body: 'TODO: một hoặc hai câu cho thói quen này.' },
    { title: '03 · Đo ít, quan sát nhiều', body: 'TODO: một hoặc hai câu cho thói quen này.' },
    { title: '04 · Buông bỏ có chọn lọc', body: 'TODO: một hoặc hai câu cho thói quen này.' },
  ],
  contact: {
    eyebrow: 'Giữ liên lạc · Get in touch',
    title: 'Nếu bạn đang tìm một kỹ sư backend.',
    body: 'TODO: viết đoạn contact — quan điểm tuyển dụng, thời gian phản hồi, và những chủ đề bạn sẵn sàng nói chuyện. Dưới 280 ký tự.',
    email: 'TODO: thay bằng email thật hoặc xoá dòng này',
    links: [
      { label: 'GitHub', url: 'https://github.com/ductran', handle: '@ductran' },
      { label: 'LinkedIn', url: 'https://www.linkedin.com/in/duc-tran-huu-167b1612a/', handle: 'duc-tran-huu' },
      { label: 'RSS / Atom', url: '/feed.xml', handle: '/feed.xml' },
    ],
    recentPost: {
      label: 'Bài viết gần nhất',
      date: 'TODO: 14 / 03 / 2025',
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
