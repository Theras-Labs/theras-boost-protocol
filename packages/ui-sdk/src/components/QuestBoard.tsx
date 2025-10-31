import React from 'react';
import { track } from '@theras/core-sdk';
import { useTheras } from '../provider/TherasProvider';

type Quest = { id: string; title: string; description?: string };

type Props = {
  quests: Quest[];
  onComplete?: (id: string) => void;
};

export function QuestBoard({ quests, onComplete }: Props) {
  const { projectKey } = useTheras();
  async function complete(id: string) {
    await track('quest_complete', { projectKey, questId: id });
    if (onComplete) onComplete(id);
  }
  return (
    <div>
      {quests.map((q) => (
        <div key={q.id}>
          <div>{q.title}</div>
          {q.description ? <div>{q.description}</div> : null}
          <button onClick={() => complete(q.id)}>Complete</button>
        </div>
      ))}
    </div>
  );
}
