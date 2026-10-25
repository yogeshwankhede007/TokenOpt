import { nativeBinding } from './index';
import { CacheConfig, CacheStats } from './types';

export class TokenCache {
  private handle: number;

  constructor(config?: Partial<CacheConfig>) {
    this.handle = nativeBinding.token_cache_new(config || {});
  }

  put<T>(key: string, namespace: string, value: T, tags?: string[]): void {
    nativeBinding.token_cache_put(this.handle, key, namespace, value, tags || []);
  }

  get<T>(key: string, namespace: string): T | null {
    return nativeBinding.token_cache_get(this.handle, key, namespace);
  }

  getOrCompute<T>(key: string, namespace: string, compute: () => T): T {
    return nativeBinding.token_cache_get_or_compute(this.handle, key, namespace, compute);
  }

  invalidateByTag(tag: string): number {
    return nativeBinding.token_cache_invalidate_by_tag(this.handle, tag);
  }

  invalidateNamespace(namespace: string): number {
    return nativeBinding.token_cache_invalidate_namespace(this.handle, namespace);
  }

  clear(): void {
    nativeBinding.token_cache_clear(this.handle);
  }

  stats(): CacheStats {
    return nativeBinding.token_cache_stats(this.handle);
  }

  hitRate(): number {
    return nativeBinding.token_cache_hit_rate(this.handle);
  }

  cleanupExpired(): number {
    return nativeBinding.token_cache_cleanup_expired(this.handle);
  }
}

// Specialized cache for token counts
export class TokenCountCache {
  private cache: TokenCache;

  constructor(config?: Partial<CacheConfig>) {
    this.cache = new TokenCache(config);
  }

  getCount(text: string, model: string): number | null {
    const key = `token_count:${model}:${text}`;
    return this.cache.get<number>(key, 'tokenopt');
  }

  setCount(text: string, model: string, count: number): void {
    const key = `token_count:${model}:${text}`;
    this.cache.put(key, 'tokenopt', count, ['token_count', model]);
  }

  getConversationCount(messages: Message[], model: string): { total: number; prompt: number; completion: number } | null {
    const serialized = JSON.stringify(messages);
    const key = `conversation_count:${model}:${serialized}`;
    return this.cache.get(key, 'tokenopt');
  }

  setConversationCount(messages: Message[], model: string, count: { total: number; prompt: number; completion: number }): void {
    const serialized = JSON.stringify(messages);
    const key = `conversation_count:${model}:${serialized}`;
    this.cache.put(key, 'tokenopt', count, ['conversation_count', model]);
  }
}

// Import Message type
import { Message } from './types';