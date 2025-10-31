import React, { useEffect, useState } from 'react';
import { getConfig } from '@theras/core-sdk';
import { useTheras } from '../provider/TherasProvider';

type Balances = { tgem: number; partner: number };

type Props = {
  placeholder?: string;
};

export function LoyaltyWallet(props: Props) {
  const { apiBaseUrl, projectKey } = useTheras();
  const [balances, setBalances] = useState<Balances | null>(null);
  useEffect(() => {
    const cfg = getConfig();
    if (!apiBaseUrl || !projectKey || !cfg?.userId) return;
    const url = `${apiBaseUrl}/wallet?projectKey=${encodeURIComponent(projectKey)}&userId=${encodeURIComponent(cfg.userId)}`;
    fetch(url)
      .then((r) => r.json())
      .then((d) => setBalances({ tgem: Number(d.tgem || 0), partner: Number(d.partner || 0) }))
      .catch(() => {});
  }, [apiBaseUrl, projectKey]);
  if (!balances) return <div>{props.placeholder || 'Loading wallet...'}</div>;
  return (
    <div>
      <div>TGEM: {balances.tgem}</div>
      <div>Partner: {balances.partner}</div>
    </div>
  );
}
