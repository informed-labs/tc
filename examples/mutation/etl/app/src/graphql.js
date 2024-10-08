export const initialize = /* GraphQL */ `
  mutation Initialize($id: String!) {
    initialize(id: $id) {
      id
      status
      message
      percentage
      createdAt
      updatedAt
      __typename
    }
  }
`;

export const subscribeInitialize = /* GraphQL */ `
  subscription SubscribeInitialize($id: String!) {
    subscribeInitialize(id: $id) {
      id
      status
      message
      percentage
      createdAt
      updatedAt
      __typename
    }
  }
`;


export const subscribeEnhance = /* GraphQL */ `
  subscription SubscribeEnhance($id: String!) {
    subscribeEnhance(id: $id) {
      id
      status
      message
      percentage
      createdAt
      updatedAt
      __typename
    }
  }
`;

export const subscribeTransform = /* GraphQL */ `
  subscription SubscribeTransform($id: String!) {
    subscribeTransform(id: $id) {
      id
      status
      message
      percentage
      createdAt
      updatedAt
      __typename
    }
  }
`;

export const subscribeLoad = /* GraphQL */ `
  subscription SubscribeLoad($id: String!) {
    subscribeLoad(id: $id) {
      id
      status
      message
      percentage
      createdAt
      updatedAt
      __typename
    }
  }
`;

export const subscribeComplete = /* GraphQL */ `
  subscription SubscribeComplete($id: String!) {
    subscribeComplete(id: $id) {
      id
      status
      message
      percentage
      createdAt
      updatedAt
      __typename
    }
  }
`;
