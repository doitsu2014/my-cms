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
        </Container>
      </Section>
      <Section tone="fresh-paper" className="about-pillars">
        <Container>
          <header className="about-pillars__header">
            <Eyebrow>{content.pillarsSection.eyebrow}</Eyebrow>
            <h2 className="display-h1">{content.pillarsSection.title}</h2>
            <p className="lead about-pillars__lead">{content.pillarsSection.lead}</p>
          </header>
          <div className="about-pillars__grid">
            {content.pillars.map((pillar) => (
              <article key={pillar.label}>
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
      <Section tone="ink" className="about-statement">
        <Container>
          <blockquote className="pull-quote"><p>{content.statement.quote}</p><cite>{content.statement.cite}</cite></blockquote>
        </Container>
      </Section>
      <Section className="about-practice">
        <Container>
          <Eyebrow>{currentLang === 'vi' ? 'Cách tôi làm việc' : 'Practice'}</Eyebrow>
          <h2 className="display-h2">{currentLang === 'vi' ? 'Thói quen nhỏ.' : 'Small habits.'}</h2>
          <ul>{content.practices.map((practice) => <li key={practice.title}><h3>{practice.title}</h3><p>{practice.body}</p></li>)}</ul>
        </Container>
      </Section>
      <Section className="about-contact">
        <Container>
          <Eyebrow>{content.contact.eyebrow}</Eyebrow>
          <h2 className="display-h2">{content.contact.title}</h2>
          <p className="lead">{content.contact.body}</p>
          <ul>
            {content.contact.email && <li><a href={`mailto:${content.contact.email}`}>{content.contact.email}</a></li>}
            {content.contact.links.filter((link) => link.url).map((link) => <li key={link.url}><a href={link.url} target="_blank" rel="noopener noreferrer">{link.label}</a></li>)}
          </ul>
        </Container>
      </Section>
    </div>
  );
};

export default AboutPage;
