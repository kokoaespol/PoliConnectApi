import { EndpointParam } from './EndpointParam';

export interface EndpointResponse {
	status: number;
	contentType: string;
	params: EndpointParam[];
}
