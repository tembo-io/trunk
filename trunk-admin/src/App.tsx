import React from 'react';
import { ClerkProvider, RedirectToSignIn, SignedIn, SignedOut } from "@clerk/clerk-react";
import { Box, ChakraProvider, StackDivider, VStack } from '@chakra-ui/react'

if (!process.env.REACT_APP_CLERK_PUBLISHABLE_KEY) {
  throw new Error("Missing Publishable Key")
}
const clerkPubKey = process.env.REACT_APP_CLERK_PUBLISHABLE_KEY;


import './App.css';
import ExtensionList from './ExtensionList';

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
  return <div>
    <VStack
      divider={<StackDivider borderColor='gray.200' />}
      spacing={4}
      align='stretch'
    >
      <ExtensionList />
    </VStack>
  </div>;
}

export default App;
