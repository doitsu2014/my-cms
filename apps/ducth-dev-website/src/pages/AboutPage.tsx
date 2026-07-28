import { useParams } from 'react-router-dom';
import Container from '../components/layout/Container';
import Section from '../components/layout/Section';
import Eyebrow from '../components/editorial/Eyebrow';
import ContentEmpty from '../components/feedback/ContentEmpty';
import { getAboutContent } from '../config/about.config';

const AboutPage = () => {
  const { lang = 'en' } = useParams<{ lang: string }>();
  const currentLang = lang === 'vi' ? 'vi' : 'en';
  const content = getAboutContent(currentLang);
  if (!content.verified) {
    return (
      <Section className="about-page about-page--unverified">
        <Container>
          <Eyebrow>{currentLang === 'vi' ? 'Về tôi · About' : 'About'}</Eyebrow>
          <h1 className="display-h1">{currentLang === 'vi' ? 'Sắp ra mắt' : 'Coming soon'}</h1>
          <ContentEmpty
            lang={currentLang}
            message={currentLang === 'vi' ? 'Trang giới thiệu sẽ được cập nhật khi nội dung được xác thực.' : 'The author profile will appear when its content is verified.'}
            href={`/${currentLang}`}
          />
        </Container>
      </Section>
    );
  }

  return (
    <div className="about-page">
      <Section className="about-hero">
        <Container>
          <Eyebrow>{content.hero.eyebrow}</Eyebrow>
          <div className="about-hero__grid">
            <div>
              <h1 className="display-h1">{content.hero.title}</h1>
              <p className="about-hero__subtitle">{content.hero.subtitle}</p>
              <p className="lead">{content.hero.body}</p>
            </div>
            <img className="about-hero__portrait" src="/images/avatar.jpg" alt={content.hero.title} />
          </div>
          <dl className="about-facts">
            {content.hero.facts.map((fact) => <div key={fact.label}><dt>{fact.label}</dt><dd>{fact.value}</dd></div>)}
          </dl>
          <div className="about-hero__pillars">
            {content.pillars.map((pillar) => (
              <article key={pillar.label} className="about-hero__pillar">
                <p className="about-pillar__number">{pillar.label}</p>
                <h2 className="about-pillar__title">
                  {pillar.title}
                  {pillar.titleAccent && (
                    <span className="about-pillar__title-accent"> {pillar.titleAccent}</span>
                  )}
                </h2>
                <p className="about-pillar__subtitle">{pillar.subtitle}</p>
                {pillar.paragraphs.map((paragraph) => <p key={paragraph} className="about-pillar__body">{paragraph}</p>)}
              </article>
            ))}
          </div>
        </Container>
      </Section>
      <Section className="about-timeline">
        <Container>
          <header className="about-timeline__header">
            <Eyebrow>{content.timeline.eyebrow}</Eyebrow>
            <h2 className="display-h1">{content.timeline.title}</h2>
            <p className="about-timeline__lead">{content.timeline.lead}</p>
          </header>
          <ol className="about-timeline__list">
            {content.timeline.entries.map((entry) => (
              <li key={entry.date} className="about-timeline__entry">
                <p className="about-timeline__date">{entry.date}</p>
                <div className="about-timeline__content">
                  <h3 className="about-timeline__title">{entry.title}</h3>
                  <p className="about-timeline__eyebrow">{entry.eyebrow}</p>
                  <p className="about-timeline__body">{entry.body}</p>
                </div>
              </li>
            ))}
          </ol>
        </Container>
      </Section>
      <Section tone="fresh-paper" className="about-pillars">
        <Container>
          <header className="about-pillars__header">
            <Eyebrow>{content.domainsSection.eyebrow}</Eyebrow>
            <h2 className="display-h1">{content.domainsSection.title}</h2>
            <p className="lead about-pillars__lead">{content.domainsSection.lead}</p>
          </header>
          <div className="about-pillars__grid">
            {content.domains.map((domain) => (
              <article key={domain.label}>
                <p className="about-pillar__number">{domain.label}</p>
                <h2 className="about-pillar__title">
                  {domain.title}
                  {domain.titleAccent && (
                    <span className="about-pillar__title-accent"> {domain.titleAccent}</span>
                  )}
                </h2>
                <p className="about-pillar__subtitle">{domain.subtitle}</p>
                {domain.paragraphs.map((paragraph) => <p key={paragraph} className="about-pillar__body">{paragraph}</p>)}
              </article>
            ))}
          </div>
        </Container>
      </Section>
      <Section tone="ink" className="about-statement">
        <Container>
          <blockquote className="pull-quote"><p>{content.statement.quote}</p><cite>{content.statement.cite}</cite></blockquote>
        </Container>
      </Section>
      <Section className="about-practice">
        <Container>
          <header className="about-practice__head">
            <div className="about-practice__intro">
              <Eyebrow>{content.practiceSection.eyebrow}</Eyebrow>
              <h2 className="display-h1">{content.practiceSection.title}</h2>
            </div>
            <p className="lead about-practice__lead">{content.practiceSection.lead}</p>
          </header>
          <ul className="about-practice__list">
            {content.practices.map((practice) => (
              <li key={practice.title}>
                <h3 className="about-practice__title">{practice.title}</h3>
                <p className="about-practice__body">{practice.body}</p>
              </li>
            ))}
          </ul>
        </Container>
      </Section>
      <Section className="about-contact">
        <Container>
          <div className="about-contact__grid">
            <div className="about-contact__intro">
              <Eyebrow>{content.contact.eyebrow}</Eyebrow>
              <h2 className="display-h1">{content.contact.title}</h2>
              <p className="about-contact__body">{content.contact.body}</p>
            </div>
            <ul className="about-contact__list">
              {content.contact.email && (
                <li className="about-contact__row">
                  <a href={`mailto:${content.contact.email}`} className="about-contact__link">
                    <span className="about-contact__label">Email</span>
                    <span className="about-contact__handle">{content.contact.email}</span>
                    <span className="about-contact__arrow" aria-hidden="true">→</span>
                  </a>
                </li>
              )}
              {content.contact.links.filter((link) => link.url).map((link) => (
                <li key={link.url} className="about-contact__row">
                  <a href={link.url} target="_blank" rel="noopener noreferrer" className="about-contact__link">
                    <span className="about-contact__label">{link.label}</span>
                    <span className="about-contact__handle">{link.handle}</span>
                    <span className="about-contact__arrow" aria-hidden="true">→</span>
                  </a>
                </li>
              ))}
              {content.contact.recentPost && (
                <li className="about-contact__row">
                  <a href={content.contact.recentPost.url} className="about-contact__link">
                    <span className="about-contact__label">{content.contact.recentPost.label}</span>
                    <span className="about-contact__handle">{content.contact.recentPost.date}</span>
                    <span className="about-contact__arrow" aria-hidden="true">→</span>
                  </a>
                </li>
              )}
            </ul>
          </div>
        </Container>
      </Section>
    </div>
  );
};

export default AboutPage;