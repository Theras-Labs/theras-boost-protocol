import posthog from 'posthog-js';

export type TherasInitOptions = {
  posthogKey: string;
  apiBaseUrl?: string;
  projectKey?: string;
  userId?: string;
  userProps?: Record<string, any>;
};

let cfg: TherasInitOptions | null = null;
let phInit = false;

function uuid(): string {
  try {
    if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
      // @ts-ignore
      return crypto.randomUUID();
    }
  } catch {}
  return Math.random().toString(36).slice(2) + Date.now().toString(36);
}

export function initTheras(options: TherasInitOptions) {
  cfg = options;
  if (typeof window !== 'undefined' && options.posthogKey && !phInit) {
    posthog.init(options.posthogKey, { api_host: 'https://app.posthog.com', autocapture: false });
    phInit = true;
    if (options.userId) {
      posthog.identify(options.userId, options.userProps || {});
    }
  }
}

export function identify(userId: string, props?: Record<string, any>) {
  if (typeof window !== 'undefined' && phInit) {
    posthog.identify(userId, props || {});
  }
  if (cfg) {
    cfg.userId = userId;
    if (props) cfg.userProps = props;
  }
}

export async function track(event: string, props: Record<string, any> = {}) {
  if (typeof window !== 'undefined' && phInit) {
    posthog.capture(event, props);
  }
  if (cfg?.apiBaseUrl) {
    try {
      const idempotencyKey = uuid();
      const ts = Date.now();
      const body = {
        event,
        props,
        projectKey: cfg.projectKey,
        userId: cfg.userId,
        idempotencyKey,
        ts,
        client: typeof navigator !== 'undefined' ? { ua: navigator.userAgent } : undefined
      };
      await fetch(cfg.apiBaseUrl + '/events', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
      });
    } catch {}
  }
}

export function getConfig() {
  return cfg;
}
