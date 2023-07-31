import { Box, Button, HStack, Stack, VStack } from "@chakra-ui/react";
import { Extension } from "./forms";

interface ExtensionCardProps {
    extension: Extension;
    // onDeleteOrder: (orderId: string) => void;
}

const ExtensionCard: React.FC<ExtensionCardProps> = ({ extension }) => {
    return (
        <Box w="100%" borderWidth='5px' borderRadius='lg' overflow='hidden'>
            <HStack>
                <VStack align="start">
                    <Box
                        mt='1'
                        fontWeight='semibold'
                        as='h4'
                        lineHeight='tight'
                    >
                        * {extension.name}
                    </Box>
                    <Box
                        mt='1'
                        as='h3'
                        lineHeight='tight'
                        noOfLines={1}
                        justifyContent="start"
                        justifyItems="start"
                        justifySelf="start"
                    >
                        - {extension.description}
                    </Box>
                </VStack>
                <Button
                    justifySelf="end"
                    justifyContent="end"
                    justifyItems="end"
                    alignSelf="end"
                    alignContent="end"
                    alignItems="end"
                >
                </Button>
            </HStack>
        </Box>
    );
};

// <button onClick={handleDeleteExtension}>Cancel</button>

export default ExtensionCard;