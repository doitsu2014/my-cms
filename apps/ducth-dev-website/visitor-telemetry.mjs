import { isIP } from 'node:net';

const MAX_IP_LENGTH = 64;
const MAX_USER_AGENT_LENGTH = 1024;

function firstHeaderValue(headers, name) {
  const value = headers?.[name] ?? headers?.[name.toLowerCase()];
  return Array.isArray(value) ? value[0] : value;
}

function validIp(value) {
  if (typeof value !== 'string') return undefined;
  const trimmed = value.trim();
  if (isIP(trimmed)) return trimmed;

  const bracketed = trimmed.match(/^\[([^\]]+)](?::\d+)?$/)?.[1];
  if (bracketed && isIP(bracketed)) return bracketed;

  const withPort = trimmed.match(/^((?:\d{1,3}\.){3}\d{1,3}):\d+$/)?.[1];
  return withPort && isIP(withPort) ? withPort : undefined;
}

function firstForwardedIp(value) {
  if (typeof value !== 'string') return undefined;
  return value
    .split(',')
    .map(validIp)
    .find(Boolean);
}

export function clientAddress(headers, peerAddress) {
  return (
    firstForwardedIp(firstHeaderValue(headers, 'x-forwarded-for')) ??
    validIp(peerAddress)
  );
}

function matchVersion(value, pattern) {
  const match = value.match(pattern);
  return match?.[1]?.slice(0, 64);
}

export function parseUserAgent(value) {
  const userAgent = typeof value === 'string' ? value.slice(0, MAX_USER_AGENT_LENGTH) : '';
  if (!userAgent) return {};

  let browser;
  if (/Edg\//.test(userAgent)) browser = ['Microsoft Edge', matchVersion(userAgent, /Edg\/([^\s]+)/)];
  else if (/OPR\//.test(userAgent)) browser = ['Opera', matchVersion(userAgent, /OPR\/([^\s]+)/)];
  else if (/Firefox\//.test(userAgent)) browser = ['Firefox', matchVersion(userAgent, /Firefox\/([^\s]+)/)];
  else if (/CriOS\//.test(userAgent)) browser = ['Chrome', matchVersion(userAgent, /CriOS\/([^\s]+)/)];
  else if (/Chrome\//.test(userAgent)) browser = ['Chrome', matchVersion(userAgent, /Chrome\/([^\s]+)/)];
  else if (/Version\/[^\s]+.*Safari\//.test(userAgent)) browser = ['Safari', matchVersion(userAgent, /Version\/([^\s]+)/)];

  let os;
  if (/Windows NT/.test(userAgent)) os = ['Windows', matchVersion(userAgent, /Windows NT ([\d.]+)/)];
  else if (/Android/.test(userAgent)) os = ['Android', matchVersion(userAgent, /Android ([\d.]+)/)];
  else if (/(?:iPhone|iPad|CPU) OS/.test(userAgent)) os = ['iOS', matchVersion(userAgent, /(?:CPU (?:iPhone )?OS|iPhone OS) ([\d_]+)/)?.replaceAll('_', '.')];
  else if (/Mac OS X/.test(userAgent)) os = ['macOS', matchVersion(userAgent, /Mac OS X ([\d_]+)/)?.replaceAll('_', '.')];
  else if (/Linux/.test(userAgent)) os = ['Linux'];

  let device;
  if (/iPad/.test(userAgent)) device = ['tablet', 'iPad'];
  else if (/iPhone/.test(userAgent)) device = ['mobile', 'iPhone'];
  else if (/Android/.test(userAgent)) device = [/Mobile/.test(userAgent) ? 'mobile' : 'tablet', 'Android'];
  else if (/(Windows|Macintosh|Linux)/.test(userAgent)) device = ['desktop', 'desktop'];

  return {
    userAgent,
    browserName: browser?.[0],
    browserVersion: browser?.[1],
    osName: os?.[0],
    osVersion: os?.[1],
    deviceType: device?.[0],
    deviceModel: device?.[1],
  };
}

export function visitorAttributes({ headers, peerAddress }) {
  const attributes = {};
  const address = clientAddress(headers, peerAddress);
  if (address) attributes['client.address'] = address.slice(0, MAX_IP_LENGTH);

  const parsed = parseUserAgent(firstHeaderValue(headers, 'user-agent'));
  if (parsed.userAgent) attributes['user_agent.original'] = parsed.userAgent;
  if (parsed.browserName) attributes['user_agent.browser.name'] = parsed.browserName;
  if (parsed.browserVersion) attributes['user_agent.browser.version'] = parsed.browserVersion;
  if (parsed.osName) attributes['os.name'] = parsed.osName;
  if (parsed.osVersion) attributes['os.version'] = parsed.osVersion;
  if (parsed.deviceType) attributes['device.type'] = parsed.deviceType;
  if (parsed.deviceModel) attributes['device.model'] = parsed.deviceModel;
  return attributes;
}
