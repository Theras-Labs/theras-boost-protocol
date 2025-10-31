import React, { createContext, useContext, useEffect, useMemo } from 'react';
import { initTheras } from '@theras/core-sdk';

type Props = {
  posthogKey: string;
  apiBaseUrl?: string;
  sdkApi?: string;
  projectKey?: string;
  userId?: string;
  userProps?: Record<string, any>;
  children: React.ReactNode;
};

type Ctx = {
  apiBaseUrl?: string;
  projectKey?: string;
};

const Ctx = createContext<Ctx>({});

export function TherasProvider(p: Props) {
  const resolvedApi = p.sdkApi ?? p.apiBaseUrl;
  useEffect(() => {
    initTheras({
      posthogKey: p.posthogKey,
      apiBaseUrl: resolvedApi,
      projectKey: p.projectKey,
      userId: p.userId,
      userProps: p.userProps
    });
  }, [p.posthogKey, resolvedApi, p.projectKey, p.userId]);
  const v = useMemo(() => ({ apiBaseUrl: resolvedApi, projectKey: p.projectKey }), [resolvedApi, p.projectKey]);
  return <Ctx.Provider value={v}>{p.children}</Ctx.Provider>;
}

export function useTheras() {
  return useContext(Ctx);
}
