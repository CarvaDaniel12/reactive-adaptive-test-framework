/**
 * React Query hooks for AI configuration (Story 13.1).
 */
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { aiApi } from '@/lib/api';
import type {
  AIStatusResponse,
  AIConfigResponse,
  ProvidersResponse,
  ConfigureAIRequest,
  ConnectionTestResult,
} from '@/types';
import { useToast } from './useToast';

/**
 * Hook to get AI status.
 */
export function useAIStatus() {
  return useQuery<AIStatusResponse>({
    queryKey: ['ai-status'],
    queryFn: () => aiApi.getStatus(),
    staleTime: 30000, // 30 seconds
  });
}

/**
 * Hook to get AI configuration.
 */
export function useAIConfig() {
  return useQuery<AIConfigResponse | null>({
    queryKey: ['ai-config'],
    queryFn: () => aiApi.getConfig(),
    staleTime: 60000, // 1 minute
    retry: false, // Don't retry on 404 (not configured)
  });
}

/**
 * Hook to get available AI providers.
 */
export function useProviders() {
  return useQuery<ProvidersResponse>({
    queryKey: ['ai-providers'],
    queryFn: () => aiApi.getProviders(),
    staleTime: 300000, // 5 minutes (providers don't change often)
  });
}

/**
 * Hook to configure AI.
 */
export function useConfigureAI() {
  const queryClient = useQueryClient();
  const { toast } = useToast();

  return useMutation<{ message: string }, Error, ConfigureAIRequest>({
    mutationFn: (data) => aiApi.configure(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['ai-status'] });
      queryClient.invalidateQueries({ queryKey: ['ai-config'] });
      toast({
        title: 'AI Configured',
        description: 'AI configuration saved successfully',
        variant: 'success',
      });
    },
    onError: (error) => {
      toast({
        title: 'Configuration Failed',
        description: error.message || 'Failed to save AI configuration',
        variant: 'error',
      });
    },
  });
}

/**
 * Hook to test AI connection.
 */
export function useTestConnection() {
  const { toast } = useToast();

  return useMutation<ConnectionTestResult, Error, ConfigureAIRequest>({
    mutationFn: (data) => aiApi.testConnection(data),
    onError: (error) => {
      toast({
        title: 'Test Failed',
        description: error.message || 'Failed to test connection',
        variant: 'error',
      });
    },
  });
}

/**
 * Hook to disable AI.
 */
export function useDisableAI() {
  const queryClient = useQueryClient();
  const { toast } = useToast();

  return useMutation<{ message: string }, Error>({
    mutationFn: () => aiApi.disable(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['ai-status'] });
      queryClient.invalidateQueries({ queryKey: ['ai-config'] });
      toast({
        title: 'AI Disabled',
        description: 'AI features have been disabled',
        variant: 'success',
      });
    },
    onError: (error) => {
      toast({
        title: 'Failed to Disable',
        description: error.message || 'Failed to disable AI',
        variant: 'error',
      });
    },
  });
}
