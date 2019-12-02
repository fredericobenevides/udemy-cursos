package com.appsdeveloperblog.app.ws.userservice;

import com.appsdeveloperblog.app.ws.ui.model.UserDetailsRequestModel;
import com.appsdeveloperblog.app.ws.ui.model.response.UserRest;

public interface UserService {

    UserRest createUser(UserDetailsRequestModel userDetails);
}
