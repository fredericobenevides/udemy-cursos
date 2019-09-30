import axios from 'axios';

export default axios.create({
  baseURL: 'https://api.yelp.com/v3/businesses',
  headers: {
    Authorization:
      'Bearer AM9aHKf0MZ1d3p8YRPFP0F7fOrytZd6tQH_NCgowNHoQfewiYHyTgestnBKf12clhH5pfLdJMOgyRMl40-ZjSimZbJaV9ka37WImlBAKdyU-CGexpxhtWSWTqr2PXXYx'
  }
});
