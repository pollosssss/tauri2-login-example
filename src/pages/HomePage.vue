<template>
    <div class="flex flex-col items-center justify-center min-h-screen bg-background p-4">
      <Card class="w-full max-w-md">
        <CardHeader>
          <CardTitle class="text-center">Home</CardTitle>
        </CardHeader>
        <CardContent class="flex flex-col items-center gap-6">
          <Avatar class="w-24 h-24">
            <AvatarImage v-if="user?.avatar" :src="user.avatar" :alt="`${user.name}'s Avatar`" />
            <AvatarFallback>{{ getInitials(user?.name || 'User') }}</AvatarFallback>
          </Avatar>

          <div class="text-center">
            <p class="text-lg font-medium">{{ user?.name || 'Unknown User' }}</p>
            <p class="text-sm text-gray-500">{{ user?.email }}</p>
            <p v-if="user?.provider" class="text-xs mt-1 bg-gray-100 px-2 py-1 rounded-full inline-block">
              {{ user.provider === 'google' ? 'Google' : 'GitHub' }}
            </p>
          </div>

          <Button variant="destructive" @click="handleLogout">
            Logout
          </Button>
        </CardContent>
      </Card>
    </div>
  </template>

  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { useRouter } from 'vue-router';
  import { Button } from '@/components/ui/button';
  import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
  import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
  import { User, logout as authLogout, getCurrentUser } from '@/services/auth';

  const router = useRouter();
  const user = ref<User | null>(null);

  onMounted(async () => {
    try {
      user.value = await getCurrentUser();
      if (!user.value) {
        router.push('/');
      }
    } catch (error) {
      console.error('Failed to get user:', error);
      router.push('/');
    }
  });

  function getInitials(name: string): string {
    return name
      .split(' ')
      .map(part => part.charAt(0))
      .join('')
      .toUpperCase()
      .substring(0, 2);
  }

  function handleLogout() {
    authLogout();
    router.push('/');
  }
  </script>

  <style scoped>
  </style>
