import React from 'react';
import { ClerkProvider, RedirectToSignIn, SignedIn, SignedOut } from "@clerk/clerk-react";
import { Box, Center, ChakraProvider, StackDivider, VStack } from '@chakra-ui/react'

if (!process.env.REACT_APP_CLERK_PUBLISHABLE_KEY) {
  throw new Error("Missing Publishable Key")
}
const clerkPubKey = process.env.REACT_APP_CLERK_PUBLISHABLE_KEY;

import RequestCard from './RequestCard';

function App() {
  return (
    // Needed to use Chakra components
    <ChakraProvider>
      <ClerkProvider publishableKey={clerkPubKey}>
        <SignedIn>
          <SignedAdmin />
        </SignedIn>
        <SignedOut>
          <RedirectToSignIn />
       </SignedOut>
      </ClerkProvider>
    </ChakraProvider>
  );
}

function SignedAdmin() {
  return (
      <Box 
        w='100vw' h='100vh'
        bgGradient='linear(to-r, gray.600, gray.900)'
        display="flex" alignItems='center' gap='2'
      >
        <Center height='100vh' width='100vw'>
          <RequestCard />
        </Center>
      </Box>
  );
}

export default App;