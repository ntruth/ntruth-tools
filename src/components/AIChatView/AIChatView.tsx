import { createSignal, createEffect, onCleanup, Show, For } from 'solid-js';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import './AIChatView.css';

interface AIChatViewProps {
  queryId: string | null;
  question: string;
  isLoading: boolean;
}

export function AIChatView(props: AIChatViewProps) {
  const [content, setContent] = createSignal('');
  const [error, setError] = createSignal<string | null>(null);
  const [isStreaming, setIsStreaming] = createSignal(false);
  
  let unlistenStart: UnlistenFn | null = null;
  let unlistenChunk: UnlistenFn | null = null;
  let unlistenEnd: UnlistenFn | null = null;
  let unlistenError: UnlistenFn | null = null;

  // 设置事件监听
  createEffect(async () => {
    // 清理旧监听器
    if (unlistenStart) unlistenStart();
    if (unlistenChunk) unlistenChunk();
    if (unlistenEnd) unlistenEnd();
    if (unlistenError) unlistenError();

    if (!props.queryId) return;

    // 重置状态
    setContent('');
    setError(null);
    setIsStreaming(true);

    try {
      unlistenStart = await listen<string>('ai-quick-start', (event) => {
        if (event.payload === props.queryId) {
          setIsStreaming(true);
        }
      });

      unlistenChunk = await listen<{ id: string; chunk: string; content: string }>('ai-quick-chunk', (event) => {
        if (event.payload.id === props.queryId) {
          setContent(event.payload.content);
        }
      });

      unlistenEnd = await listen<{ id: string; content: string }>('ai-quick-end', (event) => {
        if (event.payload.id === props.queryId) {
          setContent(event.payload.content);
          setIsStreaming(false);
        }
      });

      unlistenError = await listen<{ id: string; error: string }>('ai-quick-error', (event) => {
        if (event.payload.id === props.queryId) {
          setError(event.payload.error);
          setIsStreaming(false);
        }
      });
    } catch (e) {
      console.error('Failed to setup event listeners:', e);
      setError('Failed to connect to AI service');
    }
  });

  // 清理
  onCleanup(() => {
    if (unlistenStart) unlistenStart();
    if (unlistenChunk) unlistenChunk();
    if (unlistenEnd) unlistenEnd();
    if (unlistenError) unlistenError();
  });

  // 简单的 Markdown 渲染
  const renderMarkdown = (text: string) => {
    if (!text) return [];
    
    const lines = text.split('\n');
    const elements: { type: string; content: string; language?: string }[] = [];
    let inCodeBlock = false;
    let codeContent = '';
    let codeLanguage = '';

    for (const line of lines) {
      if (line.startsWith('```')) {
        if (!inCodeBlock) {
          inCodeBlock = true;
          codeLanguage = line.slice(3).trim();
          codeContent = '';
        } else {
          elements.push({ type: 'code', content: codeContent, language: codeLanguage });
          inCodeBlock = false;
        }
      } else if (inCodeBlock) {
        codeContent += (codeContent ? '\n' : '') + line;
      } else if (line.startsWith('# ')) {
        elements.push({ type: 'h1', content: line.slice(2) });
      } else if (line.startsWith('## ')) {
        elements.push({ type: 'h2', content: line.slice(3) });
      } else if (line.startsWith('### ')) {
        elements.push({ type: 'h3', content: line.slice(4) });
      } else if (line.startsWith('- ') || line.startsWith('* ')) {
        elements.push({ type: 'li', content: line.slice(2) });
      } else if (line.match(/^\d+\. /)) {
        elements.push({ type: 'oli', content: line.replace(/^\d+\. /, '') });
      } else if (line.trim() === '') {
        elements.push({ type: 'br', content: '' });
      } else {
        elements.push({ type: 'p', content: line });
      }
    }

    // 处理未关闭的代码块
    if (inCodeBlock && codeContent) {
      elements.push({ type: 'code', content: codeContent, language: codeLanguage });
    }

    return elements;
  };

  // 渲染内联格式
  const renderInlineText = (text: string) => {
    // 处理代码、粗体、斜体
    return text
      .replace(/`([^`]+)`/g, '<code class="inline-code">$1</code>')
      .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
      .replace(/\*([^*]+)\*/g, '<em>$1</em>');
  };

  return (
    <div class="ai-chat-view">
      <div class="ai-question">
        <span class="ai-icon">🤖</span>
        <span class="question-text">{props.question}</span>
      </div>

      <div class="ai-response">
        <Show when={error()}>
          <div class="ai-error">
            <span class="error-icon">⚠️</span>
            <span class="error-text">{error()}</span>
          </div>
        </Show>

        <Show when={!error() && (content() || isStreaming())}>
          <div class="response-content">
            <For each={renderMarkdown(content())}>
              {(element) => (
                <Show when={element.type === 'code'}>
                  <pre class={`code-block language-${element.language || 'text'}`}>
                    <div class="code-header">
                      <span class="code-language">{element.language || 'code'}</span>
                    </div>
                    <code>{element.content}</code>
                  </pre>
                </Show>
              )}
            </For>
            <For each={renderMarkdown(content())}>
              {(element) => (
                <>
                  <Show when={element.type === 'h1'}>
                    <h1 innerHTML={renderInlineText(element.content)} />
                  </Show>
                  <Show when={element.type === 'h2'}>
                    <h2 innerHTML={renderInlineText(element.content)} />
                  </Show>
                  <Show when={element.type === 'h3'}>
                    <h3 innerHTML={renderInlineText(element.content)} />
                  </Show>
                  <Show when={element.type === 'p'}>
                    <p innerHTML={renderInlineText(element.content)} />
                  </Show>
                  <Show when={element.type === 'li'}>
                    <li innerHTML={renderInlineText(element.content)} />
                  </Show>
                  <Show when={element.type === 'oli'}>
                    <li class="ordered" innerHTML={renderInlineText(element.content)} />
                  </Show>
                  <Show when={element.type === 'br'}>
                    <br />
                  </Show>
                </>
              )}
            </For>
            <Show when={isStreaming()}>
              <span class="cursor-blink">▊</span>
            </Show>
          </div>
        </Show>

        <Show when={!error() && !content() && props.isLoading}>
          <div class="ai-loading">
            <div class="loading-dots">
              <span></span>
              <span></span>
              <span></span>
            </div>
            <span class="loading-text">思考中...</span>
          </div>
        </Show>
      </div>
    </div>
  );
}

export default AIChatView;
