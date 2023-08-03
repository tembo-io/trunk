import { Box, Button, HStack, Text, Stack, VStack, Select, Input, InputGroup, InputLeftAddon, InputRightAddon, Heading, Center, Skeleton, Tag, Tooltip } from "@chakra-ui/react";
import { useState } from "react";
import { sendRequest} from "./api";
import { useAuth } from "@clerk/clerk-react";

const RequestCard: React.FC = () => {
    const [httpMethod, setHttpMethod] = useState('GET');
    const [url, setUrl] = useState<string>('');
    const [requestBody, setRequestBody] = useState<string>('');
    // Response body of the last request made
    const [response, setResponse] = useState<string>('');
    // True if a request is still pending
    const [isLoading, setIsLoading] = useState<boolean>(false);
    // True if any request was made so far, false otherwise
    const [someReqWasMade, setSomeReqWasMade] = useState<boolean>(false);
    // HTTP status of the last request made
    const [httpStatus, setHttpStatus] = useState(0);
    const { getToken } = useAuth();
  
    async function dispatchRequest(): Promise<void> {
        setIsLoading(true);
        setSomeReqWasMade(true);

        const token = await getToken();

        const { statusCode, data } = await sendRequest(url, httpMethod, requestBody, token || "");

        setHttpStatus(statusCode);
        setResponse(data);
        setIsLoading(false);
    }

    return (
        <Box maxW='32rem' borderWidth='2px' borderRadius='lg' overflow='hidden' bg='white'>
            <Box borderWidth='6px' borderRadius='xl' m={5}>
                <Center><Heading mb={4}>Trunk Auth</Heading></Center>
                <Text fontSize='xl'>
                    Send Clerk-authenticated requests from here.
                </Text>
            </Box>

            <Box borderWidth='6px' borderRadius='xl' mb={2}>
                <Text mb={1}>HTTP method</Text>
                <Select
                    value={httpMethod}
                    onChange={(e) => setHttpMethod(e.target.value)}
                    colorScheme="green"
                >
                    <option value='GET'>GET</option>
                    <option value='POST'>POST</option>
                    <option value='PUT'>PUT</option>
                    <option value='DELETE'>DELETE</option>
                </Select>
            </Box>
          
            <Box borderWidth='6px' borderRadius='xl'>
                <Text mb={1}>URL to hit</Text>
                <InputGroup size='sm'>
                    <InputLeftAddon children='https://' />
                    <Input
                        placeholder='URL'
                        value={url}
                        onChange={(e) => setUrl(e.target.value)}
                    />
                </InputGroup>
            </Box>

            <Box borderWidth='6px' borderRadius='xl' mb={2}>
                <Text mb={1}>Request Body</Text>
                <Input
                    value={requestBody}
                    onChange={(e) => setRequestBody(e.target.value)}
                />
            </Box>

            <Box borderWidth='6px' borderRadius='xl' mb={2}>
                <Text mb={1}>Response</Text>
                {
                    someReqWasMade === false ? (
                        <Box borderWidth='6px' borderRadius='xl' m={4}>
                            <Text fontSize='md' mb={1}>No response yet</Text>
                        </Box>
                    ) : (
                        <Skeleton isLoaded={!isLoading}>
                            <Box borderWidth='8px' borderRadius='xl' mb={3}>
                              <Text fontSize='md'>{response}</Text>
                            </Box>
                            <Tag size={'md'} key={'md'} variant='solid' colorScheme='teal'>
                                {httpStatus}
                            </Tag>
                        </Skeleton>
                    )
                }
            </Box>

            <Center m={2}>
                <Tooltip hasArrow 
                    label='Are you sure? You have admin permissions and could be performing a destructive action. Take caution.'
                    bg='red.600' placement='top-start'
                >
                    <Button colorScheme='red' onClick={dispatchRequest}>Send request</Button>
                </Tooltip>
            </Center>

        </Box>
      );
};

export default RequestCard;