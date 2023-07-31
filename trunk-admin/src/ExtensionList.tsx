import { useState, useEffect } from 'react';
import ExtensionCard from './ExtensionCard';
import { allExtensions } from './api';
import { Extension } from './forms';
import { Button, Center, Divider, Heading, Skeleton, Stack, VStack } from '@chakra-ui/react';

const ExtensionList: React.FC = () => {
    const [extensions, setExtensions] = useState<Extension[]>([]);
    const [isLoading, setIsLoading] = useState<boolean>(false);

    useEffect(() => {
        allExtensions();
    }, []);

    const fetchExtensions = async () => {
        try {
            setIsLoading(true);
            const fetchedExtensions = await allExtensions();
            setIsLoading(false);
            setExtensions(fetchedExtensions);
        } catch (error) {
            setIsLoading(false);
            console.error(error);
        }
    };

    const handleReload = () => {
        fetchExtensions();
    };

    return (
        <Stack spacing={4} direction='column'>
            <Stack spacing={8} direction='row' justify={"center"}>
                <Heading w="100%" borderRadius='lg' bg='green' color='white'>
                    <Center>Extensions List</Center>
                </Heading>
                <Button colorScheme='teal' variant='solid' onClick={handleReload}>
                    Reload
                </Button>
            </Stack>
            <Skeleton isLoaded={!isLoading}>
                {extensions.length === 0 ? (
                    <p>No extensions available. Have you tried reloading?</p>
                 ) : (
                    <VStack>
                        <Divider />
                        {
                            extensions.map((extension) => (
                                <ExtensionCard extension={extension} />
                            ))
                        }
                    </VStack>
                )}
            </Skeleton>
        </Stack>
    );
};

export default ExtensionList;