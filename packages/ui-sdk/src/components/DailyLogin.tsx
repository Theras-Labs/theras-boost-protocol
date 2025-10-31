import React, { useState } from 'react';
import { track } from '@theras/core-sdk';
import { useTheras } from '../provider/TherasProvider';

type Props = {
  onClaim?: () => void;
  buttonText?: string;
};

export function DailyLogin(props: Props) {
  const [pending, setPending] = useState(false);
  const { projectKey } = useTheras();
  async function onClick() {
    setPending(true);
    await track('daily_login', { projectKey });
    setPending(false);
    if (props.onClaim) props.onClaim();
  }
  return (
    <button disabled={pending} onClick={onClick}>
      {props.buttonText || (pending ? 'Claiming...' : 'Claim Daily')}
    </button>
  );
}
