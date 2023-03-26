#include <algorithm>
#include <iostream>
#include <string>
#include <chrono>
#include <vector>
#include <cmath>
#include <iomanip>
#include <queue>
#include <random>
#include <map>
#include <set>
typedef long long ll;
typedef long double ld;
using namespace std;
 
ll dist(vector<ll> a, vector<ll> b)
{
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    ll sum = 0;
    for (int i = 0; i < a.size(); i++) sum += abs(a[i] - b[i]);
    return sum;
}
int main()
{
    ios::sync_with_stdio(false);
    cin.tie(0);
    int t;
    cin >> t;
    while (t--)
    {
        int n;
        cin >> n;
        n *= 2;
        vector<ll> v(n);
        for (int i = 0; i < n; i++) cin >> v[i];
        if (n == 2)
        {
            cout << abs(v[0] - v[1]) << "\n";
            continue;
        }
        vector<ll> q1(n, 0);
        vector<ll> q2(n, -1);
        q2[n - 1] = n / 2;
        ll ans = dist(v, q1);
        if (n % 4 == 0) ans = min(ans, dist(v, q2));
        if (n == 4) ans = min(ans, dist(v, vector<ll>(n, 2)));
        cout << ans << "\n";
    }
    return 0;
}
